use crate::prelude::*;

use phf_macros::phf_ordered_map;
use Command::*;
pub static COMMANDS: phf::OrderedMap<&'static str, Command> = phf_ordered_map! {
    "run" => AnyTime(crate::bomb::cmd_run),
    "detonate" => AnyTime(crate::bomb::cmd_detonate),
    "edgework" => NeedsBomb(crate::edgework::cmd_edgework),
};

pub type Parameters<'a> = std::str::SplitWhitespace<'a>;

#[derive(Clone, Copy)]
pub enum Command {
    NeedsBomb(
        fn(
            handler: &Handler,
            ctx: &Context,
            msg: &Message,
            bomb: BombRef,
            params: Parameters<'_>,
        ) -> CommandResult,
    ),
    AnyTime(
        fn(
            handler: &Handler,
            ctx: &Context,
            msg: &Message,
            params: Parameters<'_>,
        ) -> CommandResult,
    ),
}

pub fn dispatch(handler: &Handler, ctx: &Context, msg: &Message, input: String) -> CommandResult {
    // The event handler strips a `!`, so this is the `!!` case. Route directly to the last
    // viewed module.
    if input.starts_with('!') {
        let params = input[1..].trim().split_whitespace();
        crate::bomb::need_bomb(handler, msg.channel_id)?
            .write()
            .dispatch_module_command(handler, ctx, msg, None, params)
    } else {
        let mut params = input.split_whitespace();
        if let Some(first) = params.next() {
            if first.chars().all(|c| c.is_ascii_digit()) {
                crate::bomb::need_bomb(handler, msg.channel_id)?
                    .write()
                    .dispatch_module_command(handler, ctx, msg, Some(first), params)
            } else if let Some(command) = COMMANDS.get(first) {
                match command {
                    AnyTime(f) => f(handler, ctx, msg, params),
                    NeedsBomb(f) => {
                        let bomb = crate::bomb::need_bomb(handler, msg.channel_id)?;
                        f(handler, ctx, msg, bomb, params)
                    }
                }
            } else {
                // TODO: fuzzy suggestions?
                let mut builder = MessageBuilder::new();
                builder
                    .push_mono_safe(first)
                    .push(" is not recognized as a command.");

                if let Some(bomb) = crate::bomb::get_bomb(handler, msg.channel_id) {
                    let last = bomb.read().data.get_last_view(msg.author.id).unwrap_or(0);

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

                return Err(("No such command".to_owned(), builder.build()));
            }
        } else {
            // A lone `!`. Probably not intended as a command.
            Ok(())
        }
    }
}
