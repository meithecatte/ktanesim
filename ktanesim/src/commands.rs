use crate::prelude::*;
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

pub fn dispatch(ctx: &Context, msg: &Message, cmd: String) -> Result<(), ErrorMessage> {
    if cmd.starts_with('!') {
        unimplemented!();
    } else {
        let mut parts = cmd.split_whitespace();
        if let Some(first) = parts.next() {
            if let Some(descriptor) = COMMANDS.get(first) {
                match descriptor.handler {
                    CommandHandler::AnyTime(f) => f(ctx, msg, parts),
                    _ => unimplemented!(),
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

#[derive(Clone, Copy)]
pub struct Command {
    pub handler: CommandHandler,
    pub help_header: &'static str,
    pub help: &'static str,
}

#[derive(Clone, Copy)]
pub enum CommandHandler {
    NeedsBomb(fn(ctx: &Context, msg: &Message, bomb: &Bomb, params: Parameters<'_>)),
    AnyTime(fn(ctx: &Context, msg: &Message, params: Parameters<'_>) -> Result<(), ErrorMessage>),
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
    unimplemented!();
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
                    info!("Length = {}", command.help.len());
                    e.field(command.help_header, command.help, false);
                }
            }

            e
        });

        m
    });
});

// TODO: Update this
any_time!(CMD_RUN "!run [ruleseed=`seed`] [mode=`mode`] `mission`, !run [ruleseed=`seed`] [mode=`mode`] `module or group`\u{B1}... `count`, ..."
"Arm a bomb in this channel. You can either start a *mission* (see **!missions**) or choose some modules for a customized experience (see **!modules**). You can also use the following words in place of a module name to refer to more modules at once:

- *vanilla*, *base*, *unmodded* - present in the base game.
- *mods*, *modded* - available in the Steam Workshop.
- *fantasy*, *novelty* - not yet available in the Steam Workshop.
- *needy* - can't be disarmed, must be periodically interacted with in order to avoid strikes.
- *solvable*, *regular*, *normal* - the most common type of a module.
- *boss*, *special* - must be accounted for throughout the bomb, while not being a needy, such as Forget Me Not or Souvenir.
- *ruleseedable*, *ruleseed* - modules that support procedural rule generation. See also the named parameter *ruleseed*
- *all*, *any* - all of the above.

These sets can be combined with `+` or `-`. For example, `!run mods+novelty-needy+knob 30` will randomly choose 30 modules, where each is either a needy *knob* or a non-*needy* module from the *mods* or *novelty* category. The expressions are evaluated left-to-right, so `mods+novelty+knob-needy` would first add the *knob* and then prune all the needies, making it impossible for the knob to actually appear on the bomb.

To achieve more certainty about the exact modules you will encounter, you can add `each` after the count - `!run all 1 each` will give you a taste of everything in store. Alternatively, it's also possible to use more than one pair of a module set and a count, such as `!run wires 2, hexamaze 5` or `!run solvable 30, boss+needy 1 each` (good luck).

Finally, you can add named parameters before the mission name or module list:

- *seed*, *ruleseed*, *rules* - for modules that support it, generate the rules procedurally with the specified seed - for example, `!run ruleseed=7 wires+digitalRoot+capacitor 1 each` will give new rules to the vanilla Wires, but not to the needy Capacitor or Digital Root. Seeds are positive integers smaller than 2147483647 (or, as a practical approximation, up to nine digits long). This replicates the functionality of the [Rule Seed Modifier](https://steamcommunity.com/sharedfiles/filedetails/?id=1224413364) mod.
- *timer* - choose a behavior for the timer:
  - `!run timer=8m30s ...` (or any other value) - the timer is constantly running down from the specified starting value and will speed up on a strike. This mode is the default, but specifying it explicitly allows you to control the amount of time you have.
  - `!run timer=zen ...` - the timer starts at zero and is counting up. Strikes do not affect the timer speed. The bomb will not explode in this mode unless forcibly detonated. Solving modules with this option has a smaller weight on the leaderboard.
  - `!run timer=time ...` - the timer starts counting down from five minutes. Each time a module is solved, time is added, while strikes cause time to be removed.
  
Both named parameters can be used for missions as well as custom bombs."
=> crate::bomb::cmd_run);

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
