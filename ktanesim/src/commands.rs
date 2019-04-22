use crate::prelude::*;

use phf_macros::phf_ordered_map;
use CommandHandler::*;
pub static COMMANDS: phf::OrderedMap<&'static str, CommandHandler> = phf_ordered_map! {
    "run" => AnyTime(crate::bomb::cmd_run),
    "detonate" => AnyTime(crate::bomb::cmd_detonate),
    "edgework" => NeedsBomb(crate::edgework::cmd_edgework),
};

pub type Parameters<'a> = std::str::SplitWhitespace<'a>;

#[derive(Clone, Copy)]
pub enum CommandHandler {
    NeedsBomb(
        fn(ctx: &Context, msg: &Message, bomb: BombRef, params: Parameters<'_>) -> CommandResult,
    ),
    AnyTime(fn(ctx: &Context, msg: &Message, params: Parameters<'_>) -> CommandResult),
}

pub fn dispatch(ctx: &Context, msg: &Message, cmd: String) -> CommandResult {
    // Still starts with a `!`. Route directly to the last viewed module.
    if cmd.starts_with('!') {
        let params = cmd[1..].trim().split_whitespace();
        crate::bomb::need_bomb(ctx, msg)?
            .write()
            .dispatch_module_command(ctx, msg, None, params)
    } else {
        let mut params = cmd.split_whitespace();
        if let Some(first) = params.next() {
            if let Some(handler) = COMMANDS.get(first) {
                match handler {
                    AnyTime(f) => f(ctx, msg, params),
                    NeedsBomb(f) => f(ctx, msg, crate::bomb::need_bomb(&ctx, &msg)?, params),
                }
            } else if first.chars().all(|c| c.is_ascii_digit()) {
                crate::bomb::need_bomb(ctx, msg)?
                    .write()
                    .dispatch_module_command(ctx, msg, Some(first), params)
            } else {
                let mut builder = MessageBuilder::new();
                builder
                    .push_mono_safe(first)
                    .push(" is not recognized as a command.");

                if let Some(bomb) = crate::bomb::get_bomb(&ctx, &msg) {
                    let last = bomb
                        .read()
                        .defusers
                        .get(&msg.author.id)
                        .map(|defuser| defuser.last_view)
                        .unwrap_or(0)
                        + 1;

                    builder
                        .push(
                            "Did you mean to send it to one of the modules? \
                             If so, try using two exclamation marks or a \
                             module number: `!!",
                        )
                        .push_safe(&cmd)
                        .push(format!("`, or `!{} ", last))
                        .push_safe(&cmd)
                        .push("`");
                }

                return Err(("No such command".to_owned(), builder.build()));
            }
        } else {
            // A lone `!`. Probably not intended as a command.
            Ok(())
        }
    }
}
