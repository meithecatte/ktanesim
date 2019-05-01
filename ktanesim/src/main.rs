#![feature(
    proc_macro_hygiene,
    try_blocks,
    type_ascription,
    try_trait,
    int_error_matching,
    never_type,
)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod bomb;
mod commands;
mod config;
mod edgework;
mod errors;
mod modules;
mod prelude;
mod textures;
mod timing;
mod utils;

use config::Config;
use prelude::*;
use timing::TimingHandle;
use serenity::utils::Colour;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = kankyo::load() {
        eprintln!("Couldn't load .env file: {:?}", err);
    }

    env_logger::init();
    let token = env("DISCORD_TOKEN");
    let config = Config::parse()?;
    let timing: &'static _ = Box::leak(Box::new(TimingHandle::new()));
    let handler: &'static _ = Box::leak(Box::new(Handler {
        config,
        bombs: RwLock::new(HashMap::new()),
        timing,
        presence_update_pending: AtomicBool::new(false),
    }));

    let mut client = Client::new(
        &token,
        handler,
    )?;

    // ThreadPool's clone behaves like Arc
    let pool = client.threadpool.clone();
    let http = Arc::clone(&client.cache_and_http.http);
    std::thread::spawn(move || {
        timing.processing_loop(handler, http, pool);
    });

    client.start()?;
    info!("Exiting");
    Ok(())
}

fn env(name: &'static str) -> String {
    kankyo::key(name).unwrap_or_else(|| panic!("Environment variable {} not found", name))
}

pub struct Handler {
    config: Config,
    bombs: RwLock<HashMap<ChannelId, BombRef>>,
    timing: &'static TimingHandle,
    presence_update_pending: AtomicBool,
}

impl Handler {
    pub fn schedule_presence_update(&self) {
        self.presence_update_pending.store(true, Ordering::SeqCst);
    }
}

impl EventHandler for &Handler {
    fn ready(&self, ctx: Context, event: Ready) {
        info!("Ready as {}", event.user.name);
        crate::bomb::update_presence(self, &ctx);
    }

    fn message(&self, ctx: Context, msg: Message) {
        let cmd = msg.content.trim();

        if cmd.starts_with('!') && !self.config.should_ignore_message(&msg) {
            info!("Processing command: {:?}", cmd);
            let normalized = normalize(cmd[1..].trim());
            if let Err(why) = commands::dispatch(self, &ctx, &msg, normalized) {
                utils::send_message(&ctx.http, msg.channel_id, |m| {
                    m.embed(|e| {
                        e.color(Colour::RED)
                            .title(why.title())
                            .description(why.description())
                            .footer(|ft| {
                                ft.text(&msg.author.name)
                                    .icon_url(&utils::user_avatar(&msg.author))
                            })
                    })
                });
            }
        }

        // The explosion timeout event does not have a reference to the serenity structures
        // necessary to update the bomb count in the Discord presence. Because of this, the fact
        // that an update needs to be done is just remembered, and then handled when a message is
        // received. Because the event handler for messages is also triggered on the bot's own
        // messages, the delay this causes is minimal.
        //
        // Atomically write false. If the old value was true, update.
        if self.presence_update_pending.swap(false, Ordering::SeqCst) {
            crate::bomb::update_presence(self, &ctx);
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
