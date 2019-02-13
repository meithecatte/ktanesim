#![feature(
    proc_macro_hygiene,
    fnbox,
    futures_api,
    async_await,
    await_macro,
    try_blocks,
    type_ascription,
    try_trait
)]

mod backoff;
mod bomb;
mod gateway;
mod modules;
mod prelude;
#[macro_use]
mod util_macros;

#[macro_use]
extern crate log;

use gateway::event_loop;
use modules::wires;
use prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::prelude::*;
use std::sync::Arc;

pub struct Context {
    http: serenity::http::Client,
}

fn main() {
    tokio::run_async(
        async {
            if let Err(err) = kankyo::load() {
                eprintln!("Couldn't load .env file: {:?}", err);
            }

            env_logger::init();
            let token = kankyo::key("DISCORD_TOKEN").expect("Token not present in environment");

            let http = {
                let connector = hyper_tls::HttpsConnector::new(1)
                    .expect("Cannot construct a TLS connector");
                let hyper = Arc::new(hyper::Client::builder().build(connector));
                serenity::http::Client::new(hyper, Arc::new(format!("Bot {}", token)))
                    .expect("Couldn't create http client")
            };

            let ctx = Context {
                http,
            };

            // the never return type is not handled well by await!
            #[allow(unreachable_code)]
            await!(event_loop(token, ctx));
        },
    );
}

async fn event_handler(ctx: &Context, event: Event) {
    match event {
        Event::MessageCreate(MessageCreateEvent { message }) => {
            info!("Received message: {:#?}", message);
            if message.content == "!ping" {
                let response = awaitt!(ctx.http.send_message(message.channel_id, |mut e| {
                    e.content("Pong!");
                    e.reactions::<ReactionType, _>([
                        "💥".into(),
                        "🙄".into(),
         //               EmojiId(504047360436076545).into(),
                    ].iter().cloned());
                    e
                }));
                info!("Response: {:#?}", response);
            }
        }
        _ => (),
    }
}
