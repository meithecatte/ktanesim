use crate::bomb::Bomb;
use crate::utils::send_message;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Colour, MessageBuilder};

use phf_macros::phf_ordered_map;
pub static COMMANDS: phf::OrderedMap<&'static str, Command> = phf_ordered_map! {
    "claim" => CMD_CLAIM,
    "modules" => CMD_MODULES,
    "run" => CMD_RUN,
    "help" => CMD_HELP,
};

pub fn dispatch(ctx: Context, msg: Message, cmd: String) {
    if cmd.starts_with('!') {
        unimplemented!();
    } else {
        let mut parts = cmd.split_whitespace();
        if let Some(first) = parts.next() {
            if let Some(descriptor) = COMMANDS.get(first) {
                match descriptor.handler {
                    CommandHandler::AnyTime(f) => f(ctx, &msg, parts),
                    _ => unimplemented!(),
                }
            } else {
                send_message(&ctx, msg.channel_id, |m| {
                    m.embed(|e| {
                        e.color(Colour::RED).title("No such command").description({
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
                        })
                    })
                });
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Command {
    pub handler: CommandHandler,
    pub help_header: &'static str,
    pub help: &'static str,
}

#[derive(Clone, Copy)]
pub enum CommandHandler {
    NeedsBomb(fn(ctx: Context, msg: &Message, bomb: &Bomb, params: std::str::SplitWhitespace)),
    AnyTime(fn(ctx: Context, msg: &Message, params: std::str::SplitWhitespace)),
}

macro_rules! needs_bomb {( $($arg:tt)* ) => { _command!(NeedsBomb $($arg)*); }}
macro_rules! any_time   {( $($arg:tt)* ) => { _command!(AnyTime   $($arg)*); }}

macro_rules! _command {
    ( $type:ident $name:ident $help_header:tt $help:tt => $handler:expr ) => {
        static $name: Command = Command {
            handler: CommandHandler::$type($handler),
            help_header: $help_header,
            help: $help,
        };
    };
}

any_time!(CMD_HELP "!help [`command`]"
          "This message. If followed by a command name, show help for the command."
=> |ctx, msg, params| {
    send_message(&ctx, msg.channel_id, |m| {
        m.embed(|e| {
            let search = params.collect::<Vec<_>>();

            if !search.is_empty() {
                let results = search
                    .iter()
                    .filter_map(|&query| COMMANDS.get(query))
                    .collect::<Vec<_>>();

                if results.is_empty() {
                    e.color(Colour::RED);
                    e.title("Error");
                    e.description(
                        MessageBuilder::new()
                        .push("No results for ")
                        .push_mono_safe(search.join(" "))
                        .push(".")
                        .build()
                    );
                } else {
                    e.color(Colour::BLURPLE);
                    e.title("Help search results");

                    for command in &results {
                        e.field(command.help_header, command.help, false);
                    }
                }
            } else {
                e.color(Colour::BLURPLE);
                e.title("Help");
                e.description(
"This bot simulates [Keep Talking and Nobody Explodes](https://keeptalkinggame.com) bombs right in your Discord client. Message the bot in a *Direct Message* for solo play or collaborate in server channels (the bot may only be allowed to talk in some of them).

Each module on the bomb has a number. To send a command to a specific module, prefix the command with the number. For example, `!3 view` (or `!3 v` for short) will show you a picture of the third module on a bomb. The message will also contain a handy reference for module-specific commands.

As a shorthand, you can use a double exclamation mark to refer to the module most recently viewed by you: `!!cut 5` will be equivalent to `!2 cut 5` if just issued a `!2 view`.

Here's a list of commands you can use:");

                for command in COMMANDS.values() {
                    e.field(command.help_header, command.help, false);
                }
            }

            e
        });

        m
    });
});

any_time!(CMD_RUN "!run [`mode`] `mission`, !run [`mode`, ] `module or group`+... `count`, ..."
          "Arm a bomb in this channel. You can either start a *mission* (see **!missions**) or choose some modules for a customized experience (see **!modules**). You can also use the following words in place of a module name to refer to more modules at once:

- *vanilla* - present in the base game.
- *mods*, *modded* - available in the Steam Workshop.
- *fantasy*, *novelty* - not yet available in the Steam Workshop.
- *needy* - can't be disarmed, must be periodically interacted with in order to avoid strikes.
- *solvable*, *regular* - the most common type of a module.
- *boss*, *special* - must be accounted for throughout the bomb, while not being a needy, such as Forget Me Not or Souvenir."
=> |ctx, msg, params| {
    unimplemented!();
});

any_time!(CMD_MODULES "!modules"
          "Show the list of modules available for constructing bombs."
=> |ctx, msg, params| {
    unimplemented!();
});

needs_bomb!(CMD_CLAIM "!`N` claim (see also: !`N` claimview)"
          "Claim a module. When a module is claimed, only the claimant can issue commands to it. When playing in a server channel, it's recommended to claim the modules you're solving so that work is not being duplicated."
=> |ctx, msg, bomb, params| {
    unimplemented!();
});
