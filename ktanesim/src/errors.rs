use crate::prelude::*;

pub type CommandResult = Result<(), ErrorMessage>;
pub enum ErrorMessage {
    NoBomb,
    BombAlreadyRunning {
        guild: bool,
    },
    NoLastView,
    TooManyModules,
    TrailingParameters(String),
    NoSuchGlobalCommand {
        command: String,
        input: String,
        last: Option<ModuleNumber>,
    },
    NoSuchModuleCommand {
        command: String,
        module: &'static str,
        hint: String,
    },
    // starting
    MissingModuleList,
    MissingModuleCount {
        specifier: String,
    },
    UnparseableModuleCount {
        found: String,
    },
    NeedCommaAfterModuleCount {
        found: String,
        allow_each: bool,
    },
    EmptyModuleSet(crate::bomb::starting::ModuleSet),
    UnknownModuleName(String),
    ModuleGroupStartsWithOp {
        input: String,
    },
    ModuleGroupEndsWithOp {
        input: String,
    },
    ModuleGroupTwoOpsInARow {
        input: String,
        index: usize,
    },
    RepeatedRuleSeedParameter,
    RepeatedTimerParameter,
    RuleSeedParameterTooLarge,
    RuleSeedParameterUnparseable(String),
    TimerParameterUnparseable(String),
    UnknownNamedParameter(String),
    // dispatch
    ModuleNumberZero,
    ModuleNumberTooLarge {
        num: String,
        max: ModuleNumber,
    },
    ModuleAlreadySolved(ModuleNumber),
    // Wires
    WireNumberZero,
    WireNumberTooLarge {
        max: u8,
    },
    WireNumberUnparseable(String),
    CutParameterMissing,
}

use ErrorMessage::*;

impl ErrorMessage {
    pub fn title(&self) -> &'static str {
        match self {
            NoBomb => "No bomb in this channel",
            BombAlreadyRunning { .. } => "Bomb already running",
            NoLastView => "You didn't view any modules",
            TooManyModules => "Count too large",
            TrailingParameters(_) => "Trailing parameters",
            NoSuchGlobalCommand { .. } | NoSuchModuleCommand { .. } => "No such command",
            MissingModuleList => "Missing module list",
            MissingModuleCount { .. } => "Missing count",
            UnparseableModuleCount { .. } | NeedCommaAfterModuleCount { .. } => "Syntax error",
            EmptyModuleSet(_) => "Empty module set",
            UnknownModuleName(_) => "Unknown module name",
            ModuleGroupStartsWithOp { .. }
            | ModuleGroupEndsWithOp { .. }
            | ModuleGroupTwoOpsInARow { .. } => "Syntax error",
            RepeatedRuleSeedParameter | RepeatedTimerParameter => "Repeated parameter",
            RuleSeedParameterTooLarge => "Rule seed too large",
            RuleSeedParameterUnparseable(_) => "Invalid rule seed",
            TimerParameterUnparseable(_) => "Invalid timer parameter",
            UnknownNamedParameter(_) => "Unknown named parameter",
            ModuleNumberZero => "Module numbers start at 1",
            ModuleNumberTooLarge { .. } => "Module number too large",
            ModuleAlreadySolved(_) => "Module already solved",
            WireNumberZero | WireNumberTooLarge { .. } | WireNumberUnparseable(_) => {
                "Invalid wire number"
            }
            CutParameterMissing => "Missing parameter",
        }
    }

    pub fn description(&self) -> String {
        match self {
            NoBomb => {
                // TODO: link help
                "No bomb is currently running in this channel. \
                 Check out the help for more details."
                    .to_owned()
            }
            BombAlreadyRunning { guild: true } => {
                "A bomb is already running in this channel. \
                 Join in with the fun, run your own bomb by messaging the bot in private, \
                 or suggest detonating with `!detonate` (majority has to agree)."
                    .to_owned()
            }
            BombAlreadyRunning { guild: false } => "A bomb is already running in this channel. \
                                                    If you don't want to defuse this bomb anymore, \
                                                    consider using `!detonate`."
                .to_owned(),
            NoLastView => "The `!!` prefix sends the command to the module you have last viewed. \
                           However, you didn't look at any modules on this bomb yet. \
                           Try `!cvany` to claim a random module and start defusing."
                .to_owned(),
            TooManyModules => format!(
                "I like your enthusiasm, but that's too many modules for me to handle. \
                 Could you limit yourself to {} for now?",
                crate::bomb::MAX_MODULES,
            ),
            TrailingParameters(params) => MessageBuilder::new()
                .push("Unexpected parameters: ")
                .push_mono_safe(params)
                .build(),
            NoSuchGlobalCommand {
                command,
                input,
                last,
            } => {
                // TODO: fuzzy suggestions?
                let mut builder = MessageBuilder::new();
                builder
                    .push_mono_safe(command)
                    .push(" is not recognized as a command.");

                if let Some(last) = last {
                    builder
                        .push(
                            "Did you mean to send it to one of the modules? \
                             If so, try using two exclamation marks or a \
                             module number: `!!",
                        )
                        .push_safe(&input)
                        .push(format!("`, or `!{} ", last + 1))
                        .push_safe(&input)
                        .push("`");
                }

                builder.build()
            }
            NoSuchModuleCommand {
                command,
                module,
                hint,
            } => MessageBuilder::new()
                .push_mono_safe(command)
                .push(" is not a valid command for ")
                .push(module)
                .push(". ")
                .push(hint)
                .build(),
            // TODO: link help in these starting errors?
            MissingModuleList => "`!run` needs a module list specification.".to_owned(),
            MissingModuleCount { specifier } => MessageBuilder::new()
                .push("Please specify a module count after ")
                .push_mono_safe(specifier)
                .push(".")
                .build(),
            UnparseableModuleCount { found } => MessageBuilder::new()
                .push("Expected a module count, found ")
                .push_mono_safe(found)
                .push(".")
                .build(),
            NeedCommaAfterModuleCount { found, allow_each } => {
                let mut builder = MessageBuilder::new();
                if *allow_each {
                    builder.push("Expected `each,` or `,` before ");
                } else {
                    builder.push("Expected `,` before ");
                }

                builder.push_mono_safe(found).push(".").build()
            }
            EmptyModuleSet(set) => {
                format!("The module set `{}` excludes all available modules.", set)
            }
            UnknownModuleName(name) => {
                // TODO: link list
                // TODO: fuzzy suggestions?
                MessageBuilder::new()
                    .push_mono_safe(name)
                    .push(" is not recognized as a module or module group name.")
                    .build()
            }
            ModuleGroupStartsWithOp { input } => MessageBuilder::new()
                .push("The module group specifier ")
                .push_mono_safe(input)
                .push(" starts with an operator. This has no defined meaning.")
                .build(),
            ModuleGroupEndsWithOp { input } => MessageBuilder::new()
                .push("The module group specifier ")
                .push_mono_safe(input)
                .push(" ends with an operator. This has no defined meaning.")
                .build(),
            ModuleGroupTwoOpsInARow { input, index } => MessageBuilder::new()
                .push("The module group specifier ")
                .push_mono_safe(input)
                .push(" contains two operators without a module or group between them, after ")
                .push_mono_safe(&input[0..*index])
                .push(". This has no defined meaning.")
                .build(),
            RepeatedRuleSeedParameter => "Please provide only one `ruleseed` parameter.".to_owned(),
            RepeatedTimerParameter => "Please provide only one `timer` parameter.".to_owned(),
            RuleSeedParameterTooLarge => format!(
                "Please limit yourself to seeds not larger than {}.",
                ktane_utils::random::MAX_VALUE,
            ),
            RuleSeedParameterUnparseable(input) => MessageBuilder::new()
                .push_mono_safe(input)
                .push(" is not a valid rule seed. Try using `random` or a number.")
                .build(),
            TimerParameterUnparseable(input) => MessageBuilder::new()
                .push_mono_safe(input)
                .push(
                    " is not a valid argument for `timer`. Try *zen*, *time*, \
                     a duration such as `8m30s` or omit the argument.",
                )
                .build(),
            UnknownNamedParameter(name) => MessageBuilder::new()
                .push_mono_safe(name)
                .push(" is not a valid parameter name. Try `timer` or `ruleseed`.")
                .build(),
            ModuleNumberZero => {
                "`0` is not a valid module number. The numbering starts at 1.".to_owned()
            }
            ModuleNumberTooLarge { num, max } => {
                if *max == 1 {
                    format!(
                        "`{}` is not a valid module number. \
                         There is only one module on this bomb.",
                        num,
                    )
                } else {
                    format!(
                        "`{}` is not a valid module number. \
                         There are only {} modules on this bomb.",
                        num, max,
                    )
                }
            }
            ModuleAlreadySolved(num) => format!(
                "Module #{} has already been solved. \
                 Try `!cvany` to claim a new module.",
                num + 1,
            ),
            WireNumberZero => "Wire numbering starts at one.".to_owned(),
            WireNumberTooLarge { max } => format!("This module has only {} wires.", max),
            WireNumberUnparseable(input) => MessageBuilder::new()
                .push_mono_safe(input)
                .push(" is not a number. A wire number needs to be a number. duh.")
                .build(),
            CutParameterMissing => {
                "The `cut` command needs a wire number as a parameter.".to_owned()
            }
        }
    }
}
