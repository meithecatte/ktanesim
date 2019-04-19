use crate::prelude::*;
use serenity::utils::MessageBuilder;

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
        fn(
            ctx: &Context,
            msg: &Message,
            bomb: Arc<RwLock<Bomb>>,
            params: Parameters<'_>,
        ) -> CommandResult,
    ),
    AnyTime(fn(ctx: &Context, msg: &Message, params: Parameters<'_>) -> CommandResult),
}

pub fn dispatch(ctx: &Context, msg: &Message, cmd: String) -> CommandResult {
    if cmd.starts_with('!') {
        unimplemented!();
    } else {
        let mut parts = cmd.split_whitespace();
        if let Some(first) = parts.next() {
            if let Some(handler) = COMMANDS.get(first) {
                match handler {
                    AnyTime(f) => f(ctx, msg, parts),
                    NeedsBomb(f) => {
                        if let Some(bomb) = crate::bomb::get_bomb(&ctx, &msg) {
                            f(ctx, msg, bomb, parts)
                        } else {
                            crate::bomb::no_bomb()
                        }
                    }
                }
            } else {
                return Err(("No such command".to_owned(), {
                    let mut builder = MessageBuilder::new();
                    builder
                        .push_mono_safe(first)
                        .push(" is not recognized as a command.");

                    if let Some(bomb) = crate::bomb::get_bomb(&ctx, &msg) {
                        let last = bomb
                            .read()
                            .defusers
                            .get(&msg.author.id)
                            .and_then(|defuser| defuser.last_view)
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

                    builder.build()
                }));
            }
        } else {
            Ok(())
        }
    }
}
