use crate::bomb::{Timer, TimerMode, MAX_MODULES};
use crate::modules::ModuleGroup;
use crate::prelude::*;
use crate::utils::{ranged_int_parse, RangedIntError};
use itertools::Itertools;
use rand::prelude::*;
use serenity::utils::Colour;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

fn bomb_already_running(msg: &Message) -> Result<!, ErrorMessage> {
    Err(ErrorMessage::BombAlreadyRunning { guild: msg.guild_id.is_some()})
}

fn ensure_no_bomb(handler: &Handler, msg: &Message) -> Result<(), ErrorMessage> {
    if crate::bomb::running_in(handler, msg.channel_id) {
        bomb_already_running(msg)?
    } else {
        Ok(())
    }
}

fn start_bomb(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
    timer: TimerMode,
    rule_seed: u32,
    modules: &[&'static ModuleDescriptor],
) -> CommandResult {
    use std::collections::hash_map::Entry;
    let solvable_count = modules
        .iter()
        .filter(|descriptor| descriptor.category != ModuleCategory::Needy)
        .count() as ModuleNumber;
    let render;
    // TODO: use upgradable read
    match handler.bombs.write().entry(msg.channel_id) {
        // the starting commands make sure that there is no bomb running, but commands are
        // processed across multiple threads...
        Entry::Occupied(_) => bomb_already_running(msg)?,
        Entry::Vacant(entry) => {
            let mut data = BombData {
                edgework: random(),
                rule_seed,
                module_count: modules.len() as ModuleNumber,
                solvable_count,
                solved_count: 0,
                timer: Timer::new(timer),
                channel: msg.channel_id,
                defusers: HashMap::new(),
                drop_callback: None,
            };

            let modules = modules
                .iter()
                .enumerate()
                .map(|(num, module)| {
                    (module.constructor)(&mut data, ModuleState::new(num as ModuleNumber))
                })
                .collect();

            let bomb = Bomb { modules, data };
            render = bomb.data.render_edgework();
            entry.insert(Arc::new(RwLock::new(bomb)));
            info!("Bomb armed");
        }
    }

    render.resolve(ctx, msg.channel_id, |m, file| {
        m.embed(|e| {
            e.color(Colour::DARK_GREEN)
                .title("Bomb armed")
                .description(format!(
                    "A {}-module bomb has been armed! \
                     Type `!cvany` to claim one of the modules and start defusing!",
                    modules.len(),
                ))
                .image(file)
        })
    });

    crate::bomb::update_presence(handler, ctx);

    Ok(())
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
    Each(ModuleNumber),
    Total(ModuleNumber),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleSet {
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

pub fn cmd_run(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
    params: Parameters<'_>,
) -> CommandResult {
    ensure_no_bomb(handler, msg)?;

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
        let group = parse_group(specifier)?;
        let count = parts.next()
            .ok_or_else(|| ErrorMessage::MissingModuleCount {
                specifier: specifier.to_owned()
            })?;

        let count: ModuleNumber = match ranged_int_parse(count, MAX_MODULES) {
            Ok(count) => count,
            Err(RangedIntError::TooLarge) => return Err(ErrorMessage::TooManyModules),
            Err(_) => return Err(ErrorMessage::UnparseableModuleCount {
                found: count.to_owned(),
            }),
        };

        let count: ModuleCount = match parts.next() {
            Some("each") => ModuleCount::Each(count),
            None => ModuleCount::Total(count),
            Some(other) => {
                return Err(ErrorMessage::NeedCommaAfterModuleCount {
                    found: other.to_owned(),
                    allow_each: true,
                });
            }
        };

        if let Some(unexpected_token) = parts.next() {
            return Err(ErrorMessage::NeedCommaAfterModuleCount {
                found: unexpected_token.to_owned(),
                allow_each: false,
            });
        }

        module_groups.push((group, count));
    }

    // TODO: link help
    if module_groups.is_empty() {
        return Err(ErrorMessage::MissingModuleList);
    }

    let modules = choose_modules(&module_groups)?;
    assert!(!modules.is_empty());
    let (strikes, time) = calculate_settings(&modules);
    let timer = match named.timer {
        Some(TimerMode::Normal { time, .. }) => TimerMode::Normal { time, strikes },
        Some(timer) => timer,
        None => TimerMode::Normal { time, strikes },
    };

    start_bomb(handler, ctx, msg, timer, named.rule_seed, &modules)
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
                if chosen_modules.len() + count as usize * set_modules.len() > MAX_MODULES as usize
                {
                    return Err(ErrorMessage::TooManyModules);
                }

                for _ in 0..count {
                    chosen_modules.extend_from_slice(&set_modules);
                }
            }
            ModuleCount::Total(count) => {
                if chosen_modules.len() + count as usize > MAX_MODULES as usize {
                    return Err(ErrorMessage::TooManyModules);
                }

                for _ in 0..count {
                    chosen_modules.push(set_modules.choose(rng)
                        .ok_or(ErrorMessage::EmptyModuleSet(set.clone()))?);
                }
            }
        }

        // fail-safe, should not happen
        if chosen_modules.len() > MAX_MODULES as usize {
            return Err(ErrorMessage::TooManyModules);
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
    fn get_group(name: &str) -> Result<ModuleGroup, ErrorMessage> {
        crate::modules::MODULE_GROUPS.get(name).copied()
            .ok_or_else(|| ErrorMessage::UnknownModuleName(name.to_owned()))
    }

    let mut separators = input
        .match_indices(|c| match c {
            '+' | '-' => true,
            _ => false,
        })
        .peekable();

    let base = match separators.peek() {
        Some(&(0, _)) => return Err(ErrorMessage::ModuleGroupStartsWithOp {
            input: input.to_owned(),
        }),
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
                    return Err(ErrorMessage::ModuleGroupTwoOpsInARow {
                        input: input.to_owned(),
                        index: left,
                    });
                }

                name
            }
            None => {
                let name = &input[start..];

                if name.is_empty() {
                    return Err(ErrorMessage::ModuleGroupEndsWithOp {
                        input: input.to_owned(),
                    });
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
                    return Err(ErrorMessage::RepeatedRuleSeedParameter);
                }
            }
            NamedParameter::Timer(specifier) => {
                if timer.replace(specifier).is_some() {
                    return Err(ErrorMessage::RepeatedTimerParameter);
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
            if value == "random" {
                // 0 is a valid seed, let's try to generate it.
                let mut seed = rand::thread_rng().gen_range(1, MAX_VALUE + 1);

                if seed == 1 {
                    seed = 0;
                }

                info!("Chose rule seed: {:?}", seed);

                Ok(NamedParameter::Ruleseed(seed))
            } else {
                match ranged_int_parse(value, MAX_VALUE) {
                    Ok(seed) => Ok(NamedParameter::Ruleseed(seed)),
                    Err(RangedIntError::TooLarge) => Err(ErrorMessage::RuleSeedParameterTooLarge),
                    Err(_) => Err(ErrorMessage::RuleSeedParameterUnparseable(value.to_owned())),
                }
            }
        }
        "timer" => {
            if let Ok(mode) = value.parse() {
                Ok(NamedParameter::Timer(mode))
            } else if let Ok(time) = humantime::parse_duration(value) {
                Ok(NamedParameter::Timer(TimerMode::Normal {
                    time,
                    strikes: 0,
                }))
            } else {
                Err(ErrorMessage::TimerParameterUnparseable(value.to_owned()))
            }
        }
        _ => Err(ErrorMessage::UnknownNamedParameter(value.to_owned())),
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
