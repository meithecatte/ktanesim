#![feature(
    proc_macro_hygiene,
    fnbox,
    try_blocks,
    type_ascription,
    try_trait,
    int_error_matching,
    never_type,
    iter_copied
)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod bomb;
mod commands;
mod config;
mod edgework;
mod modules;
mod prelude;
mod utils;

use config::Config;
use prelude::*;
use serenity::utils::Colour;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = kankyo::load() {
        eprintln!("Couldn't load .env file: {:?}", err);
    }

    env_logger::init();
    let token = env("DISCORD_TOKEN");
    let config = Config::parse()?;
    let mut client = Client::new(&token, Handler { config })?;
    client.data.write().insert::<bomb::Bombs>(HashMap::new());
    client.start()?;
    Ok(())
}

fn env(name: &'static str) -> String {
    kankyo::key(name).unwrap_or_else(|| panic!("Environment variable {} not found", name))
}

struct Handler {
    config: Config,
}

impl EventHandler for Handler {
    fn ready(&self, _: Context, event: Ready) {
        info!("Ready as {}", event.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        let cmd = msg.content.trim();

        if !cmd.starts_with('!') || self.config.should_ignore_message(&msg) {
            return;
        }

        info!("Processing command: {:?}", cmd);
        let normalized = normalize(cmd[1..].trim());
        if let Err((title, description)) = commands::dispatch(&ctx, &msg, normalized) {
            send_message(&ctx, msg.channel_id, |m| {
                m.embed(|e| e.color(Colour::RED).title(title).description(description))
            });
        }
    }
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
