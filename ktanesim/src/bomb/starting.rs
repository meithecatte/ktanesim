use super::Timer;
use crate::bomb::{Bombs, TimerMode};
use crate::modules::ModuleGroup;
use crate::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use serenity::utils::{Colour, MessageBuilder};
use smallbitvec::SmallBitVec;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

const MAX_MODULES: u32 = 101;

fn ensure_no_bomb(ctx: &Context, msg: &Message) -> Result<(), ErrorMessage> {
    if crate::bomb::running_in(ctx, msg) {
        return Err((
            "Bomb already running".to_owned(),
            if msg.guild_id.is_some() {
                "A bomb is already running in this channel. Join in with the fun, run your own bomb by messaging the bot in private, or suggest detonating with `!detonate` (majority has to agree).".to_owned()
            } else {
                "A bomb is already running in this channel. If you don't want to defuse this bomb anymore, consider using `!detonate`.".to_owned()
            },
        ));
    }

    Ok(())
}

fn start_bomb(
    ctx: &Context,
    msg: &Message,
    timer: TimerMode,
    strikes: u32,
    rule_seed: u32,
    modules: &[&'static ModuleDescriptor],
) {
    let mut bomb = Bomb {
        edgework: random(),
        rule_seed,
        timer: Timer::new(timer),
        modules: vec![],
        solve_state: SmallBitVec::from_elem(modules.len(), false),
        channel: msg.channel_id,
        defusers: HashMap::new(),
    };

    for module in modules {
        let module = (module.constructor)(&mut bomb);
        bomb.modules.push(module);
    }

    let render = bomb.render_edgework();

    assert!(ctx
        .data
        .write()
        .get_mut::<Bombs>()
        .and_then(|bombs| bombs.insert(msg.channel_id, Arc::new(RwLock::new(bomb))))
        .is_none());

    render.resolve(ctx, msg.channel_id, |m, file| {
        m.embed(|e| {
            e.color(Colour::DARK_GREEN)
                .title("Bomb armed")
                .description(format!(
                    "A bomb with {} module{} has been armed! \
                     Type `!cvany` to claim one of the modules and start defusing!",
                    modules.len(),
                    if modules.len() == 1 { "" } else { "s" }
                ))
                .image(file)
        })
    });

    crate::bomb::update_presence(ctx);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SetOperation {
    Add,
    Remove,
}

impl SetOperation {
    fn as_char(self) -> char {
        match self {
            SetOperation::Add => '+',
            SetOperation::Remove => '-',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModuleCount {
    Each(u32),
    Total(u32),
}

#[derive(Clone, Debug, PartialEq)]
struct ModuleSet {
    base: ModuleGroup,
    operations: Cow<'static, [(SetOperation, ModuleGroup)]>,
}

impl ModuleSet {
    fn evaluate(&self) -> HashSet<&'static ModuleDescriptor> {
        let mut set = HashSet::new();
        self.base.add_to_set(&mut set);

        for (operation, group) in self.operations.as_ref() {
            match operation {
                SetOperation::Add => group.add_to_set(&mut set),
                SetOperation::Remove => group.remove_from_set(&mut set),
            };
        }

        set
    }
}

impl fmt::Display for ModuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)?;

        for (operation, group) in self.operations.iter() {
            write!(f, "{}{}", operation.as_char(), group)?;
        }

        Ok(())
    }
}

pub fn cmd_run(ctx: &Context, msg: &Message, params: Parameters<'_>) -> CommandResult {
    ensure_no_bomb(&ctx, &msg)?;

    let mut params = params.peekable();
    let mut named = Vec::new();

    while let Some(index) = params.peek().and_then(|param| param.find('=')) {
        let (name, value) = params.next().unwrap().split_at(index);
        named.push(get_named_parameter(name, &value[1..])?);
    }

    let named = consolidate_named_parameters(named.into_iter())?;

    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"([a-z+-]) ([a-z+-])").unwrap();
    };

    let mut module_groups = vec![];

    for group in params
        .join(" ")
        .split(',')
        .map(str::trim)
        .filter(|param| !param.is_empty())
        .map(|param| REGEX.replace_all(param, "$1$2"))
    {
        let mut parts = group.split(' ');
        let specifier = parts.next().unwrap(); // always Some because we filter on is_empty
        let count = parts.next().ok_or_else(|| {
            (
                "Missing count".to_owned(),
                MessageBuilder::new()
                    .push("Please specify a count after ")
                    .push_mono_safe(specifier)
                    .build(),
            )
        })?;

        let count: u32 = match count.parse() {
            Ok(count) if count <= MAX_MODULES => count,
            Err(ref why) if why.kind() != &core::num::IntErrorKind::Overflow => {
                return Err((
                    "Syntax error".to_owned(),
                    MessageBuilder::new()
                        .push("Expected a count, found ")
                        .push_mono_safe(count)
                        .build(),
                ));
            }
            _ => {
                return Err((
                    "Count too large".to_owned(),
                    format!(
                        "I like your enthusiasm, but don't you think that's a bit too many \
                         modules? Could you limit yourself to {} for now?",
                        MAX_MODULES
                    ),
                ));
            }
        };

        let count = match parts.next() {
            Some("each") => ModuleCount::Each(count),
            None => ModuleCount::Total(count),
            Some(other) => {
                return Err((
                    "Syntax error".to_owned(),
                    MessageBuilder::new()
                        .push("Expected `each` or a comma, found ")
                        .push_mono_safe(other)
                        .build(),
                ));
            }
        };

        if let Some(garbage) = parts.next() {
            return Err((
                "Trailing arguments".to_owned(),
                MessageBuilder::new()
                    .push("Expected a comma or message end, found ")
                    .push_mono_safe(garbage)
                    .build(),
            ));
        }

        module_groups.push((parse_group(specifier)?, count));
    }

    // TODO: link help
    if module_groups.is_empty() {
        return Err((
            "Missing module list".to_owned(),
            "Please specify a module list. See the help for more details.".to_owned(),
        ));
    }

    let modules = choose_modules(&module_groups)?;
    assert!(!modules.is_empty());
    let (strikes, timer) = calculate_settings(&modules);
    let timer = named.timer.unwrap_or_else(|| TimerMode::Normal(timer));

    start_bomb(ctx, msg, timer, strikes, named.rule_seed, &modules);

    Ok(())
}

fn choose_modules(
    sets: &[(ModuleSet, ModuleCount)],
) -> Result<Vec<&'static ModuleDescriptor>, ErrorMessage> {
    let mut chosen_modules = vec![];
    let rng = &mut rand::thread_rng();

    for (set, count) in sets {
        let set_modules = set.evaluate().iter().copied().collect::<Vec<_>>();

        match *count {
            ModuleCount::Each(count) => {
                for _ in 0..count {
                    chosen_modules.extend_from_slice(&set_modules);
                }
            }
            ModuleCount::Total(count) => {
                for _ in 0..count {
                    chosen_modules.push(set_modules.choose(rng).ok_or_else(|| {
                        (
                            "Empty module set".to_owned(),
                            MessageBuilder::new()
                                .push("The module set specifier ")
                                .push_mono_safe(set.to_string())
                                .push(" excludes all available modules.")
                                .build(),
                        )
                    })?);
                }
            }
        }
    }

    Ok(chosen_modules)
}

/// Given a module list, return the default strike count and timer length.
fn calculate_settings(modules: &[&'static ModuleDescriptor]) -> (u32, Duration) {
    let total_count = modules.len();
    let vanilla_count = modules
        .iter()
        .filter(|module| module.origin == ModuleOrigin::Vanilla)
        .count();

    // Shamelessly aped from Twitch Plays
    let strikes = (total_count / 10).max(3);

    // A minute for vanillas, two for anything else
    let minutes = 2 * total_count - vanilla_count;

    (strikes as u32, Duration::from_secs(60 * minutes as u64))
}

fn parse_group(input: &str) -> Result<ModuleSet, ErrorMessage> {
    fn no_meaning(
        input: &str,
        f: impl FnOnce(&mut MessageBuilder) -> &mut MessageBuilder,
    ) -> Result<!, ErrorMessage> {
        let mut builder = MessageBuilder::new();
        builder
            .push("The module group specifier ")
            .push_mono_safe(input);
        f(&mut builder);
        let msg = builder.push(". This has no defined meaning.").build();
        Err(("Syntax error".to_owned(), msg))
    }

    // TODO: link list on website
    fn get_group(name: &str) -> Result<ModuleGroup, ErrorMessage> {
        crate::modules::MODULE_GROUPS.get(name).copied().ok_or_else(|| (
            "No such module".to_owned(),
            MessageBuilder::new()
            .push_mono_safe(name)
            .push(" is not recognized as a module or module group name. Try **!modules** to get a list.")
            .build()))
    }

    let mut separators = input
        .match_indices(|c| match c {
            '+' | '-' => true,
            _ => false,
        })
        .peekable();

    let base = match separators.peek() {
        Some(&(0, _)) => no_meaning(input, |m| m.push(" starts with an operator"))?,
        Some(&(index, _)) => get_group(&input[0..index])?,
        None => {
            return Ok(ModuleSet {
                base: get_group(input)?,
                operations: Cow::Borrowed(&[]),
            });
        }
    };

    let mut operations = vec![];

    while let Some((left, op)) = separators.next() {
        let start = left + 1;
        let name = match separators.peek() {
            Some(&(end, _)) => {
                let name = &input[start..end];

                if name.is_empty() {
                    no_meaning(input, |m| {
                        m
                            .push(" contains two operators without a module or group between them, after ")
                            .push_mono_safe(&input[..left])
                    })?;
                }

                name
            }
            None => {
                let name = &input[start..];

                if name.is_empty() {
                    no_meaning(input, |m| m.push(" ends with an operator"))?;
                }

                name
            }
        };

        let op = match op {
            "+" => SetOperation::Add,
            "-" => SetOperation::Remove,
            _ => unreachable!(),
        };

        operations.push((op, get_group(name)?));
    }

    Ok(ModuleSet {
        base,
        operations: Cow::from(operations),
    })
}

/// The value of a single named parameter.
enum NamedParameter {
    Ruleseed(u32),
    Timer(TimerMode),
}

/// A list of values for all named parameters
struct NamedParameters {
    rule_seed: u32,
    timer: Option<TimerMode>,
}

fn consolidate_named_parameters(
    params: impl Iterator<Item = NamedParameter>,
) -> Result<NamedParameters, ErrorMessage> {
    let mut rule_seed = None;
    let mut timer = None;

    for param in params {
        match param {
            NamedParameter::Ruleseed(seed) => {
                if rule_seed.replace(seed).is_some() {
                    return Err((
                        "Repeated parameter".to_owned(),
                        "It does not make sense to specify more than one rule seed.".to_owned(),
                    ));
                }
            }
            NamedParameter::Timer(specifier) => {
                if timer.replace(specifier).is_some() {
                    return Err((
                        "Repeated parameter".to_owned(),
                        "It does not make sense to specify more than one timer value.".to_owned(),
                    ));
                }
            }
        }
    }

    Ok(NamedParameters {
        rule_seed: rule_seed.unwrap_or(ktane_utils::random::VANILLA_SEED),
        timer,
    })
}

fn get_named_parameter(name: &str, value: &str) -> Result<NamedParameter, ErrorMessage> {
    match name {
        "ruleseed" | "seed" | "rules" => {
            use ktane_utils::random::MAX_VALUE;
            match value.parse() {
                Ok(seed) => {
                    if seed <= MAX_VALUE {
                        Ok(NamedParameter::Ruleseed(seed))
                    } else {
                        Err((
                            "Seed too large".to_owned(),
                            format!(
                                "Please limit yourself to seeds not larger than {}",
                                MAX_VALUE
                            ),
                        ))
                    }
                }
                Err(_) => Err((
                    "Couldn't parse argument".to_owned(),
                    MessageBuilder::new()
                        .push_mono_safe(value)
                        .push(" is not a valid rule seed. Try using a natural number.")
                        .build(),
                )),
            }
        }
        "timer" => {
            if let Ok(mode) = value.parse() {
                Ok(NamedParameter::Timer(mode))
            } else if let Ok(time) = humantime::parse_duration(value) {
                Ok(NamedParameter::Timer(TimerMode::Normal(time)))
            } else {
                Err((
                    "Not a valid timer value".to_owned(),
                    MessageBuilder::new()
                        .push_mono_safe(value)
                        .push(
                            " is not a valid argument for `timer`. Try *zen*, *time*, \
                             a duration such as `8m30s` or omit the argument.",
                        )
                        .build(),
                ))
            }
        }
        _ => Err((
            "Unknown parameter".to_owned(),
            MessageBuilder::new()
                .push_mono_safe(value)
                .push(
                    " is not a valid argument name. Try *timer* or *ruleseed* \
                     (alias *seed* or *rules*).",
                )
                .build(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_group_test() {
        assert_eq!(parse_group("thing"),
            Err((
                "No such module".to_owned(),
                "`thing` is not recognized as a module or module group name. Try **!modules** to get a list."
                .to_owned()
            ))
        );

        for valid_single in &[
            "wires",
            "wires+wires",
            "wires+wires+wires",
            "wires-wires+wires",
        ] {
            let result = parse_group(valid_single).unwrap().evaluate();
            assert_eq!(result.len(), 1);
            assert_eq!(
                result.into_iter().next(),
                Some(&crate::modules::wires::DESCRIPTOR),
            );
        }

        for valid_empty in &["wires-wires", "wires-all", "wires+wires-wires"] {
            let result = parse_group(valid_empty).unwrap().evaluate();
            assert_eq!(result.len(), 0);
        }

        assert_eq!(parse_group("+"), Err((
            "Syntax error".to_owned(),
            "The module group specifier `+` starts with an operator. This has no defined meaning.".to_owned(),
        )));

        assert_eq!(parse_group("wires+"), Err((
            "Syntax error".to_owned(),
            "The module group specifier `wires+` ends with an operator. This has no defined meaning.".to_owned(),
        )));

        assert_eq!(parse_group("wires+-mods"), Err((
            "Syntax error".to_owned(),
            "The module group specifier `wires+-mods` contains two operators without a module or \
             group between them, after `wires`. This has no defined meaning.".to_owned(),
        )));
    }
}
