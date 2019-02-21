#![feature(proc_macro_hygiene, fnbox, try_blocks, type_ascription, try_trait)]

#[macro_use]
extern crate log;

use prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::sync::Arc;

mod bomb;
mod commands;
mod modules;
mod prelude;
mod utils;

fn env(name: &'static str) -> String {
    kankyo::key(name).unwrap_or_else(|| panic!("Environment variable {} not found", name))
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = kankyo::load() {
        eprintln!("Couldn't load .env file: {:?}", err);
    }

    env_logger::init();
    let token = env("DISCORD_TOKEN");
    let config = Config::parse()?;
    let mut client = Client::new(&token, Handler { config })?;
    client.start()?;
    Ok(())
}

struct Config {
    allow_dm: bool,
    bot_owner: UserId,
    allowed_channels: HashSet<ChannelId>,
}

impl Config {
    fn parse() -> Result<Config, Box<dyn Error>> {
        Ok(Config {
            allow_dm: env("ALLOW_DM").parse()?,
            bot_owner: UserId(env("BOT_OWNER").parse()?),
            allowed_channels: env("ALLOWED_CHANNELS")
                .split(',')
                .map(|s| s.parse().map(|id| ChannelId(id)))
                .collect::<Result<_, _>>()?,
        })
    }

    fn should_ignore_message(&self, msg: &Message) -> bool {
        if msg.author.bot {
            return true;
        }

        if msg.guild_id.is_some() {
            !self.allowed_channels.contains(&msg.channel_id)
        } else {
            !self.allow_dm && msg.author.id != self.bot_owner
        }
    }
}

struct Handler {
    config: Config,
}

/// Convert the string to lowercase while replacing some unicode homoglyphs that keyboards tend to
/// produce.
fn normalize(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '`' | '\u{2018}' | '\u{2019}' | '\u{2032}' => output.push('\''),
            _ => {
                for converted in c.to_lowercase() {
                    output.push(converted);
                }
            }
        }
    }

    output
}

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, event: Ready) {
        info!("Ready as {}", event.user.name);
    }

    fn message(&self, ctx: Context, message: Message) {
        use commands::{CommandHandler, COMMANDS};
        let cmd = message.content.trim();

        if !cmd.starts_with('!') || self.config.should_ignore_message(&message) {
            return;
        }

        info!("Processing command: {:?}", cmd);

        let cmd = normalize(cmd[1..].trim());
        let mut parts = cmd.split_whitespace();
        if let Some(first) = parts.next() {
            trace!("Verb: {:?}", first);
            if let Some(descriptor) = COMMANDS.get(first) {
                match descriptor.handler {
                    CommandHandler::AnyTime(f) => f(ctx, &message, parts),
                    _ => unimplemented!(),
                }
            }
        }
    }
}
