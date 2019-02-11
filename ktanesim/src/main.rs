#![feature(futures_api, async_await, await_macro, try_blocks)]
#![feature(try_trait)]

#[macro_use] extern crate log;

use ktanesim::modules::wires;
use ktanesim::prelude::*;
use serenity::gateway::Shard;
use std::io::prelude::*;
use tokio::prelude::*;
use tokio_async_await::compat::forward::IntoAwaitable;
use tungstenite::error::Error as TungsteniteError;
use serenity::prelude::*;

#[derive(Debug)]
enum Error {
    None,
    Tungstenite(TungsteniteError),
    Serenity(SerenityError),
}

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Error {
        Error::None
    }
}

impl From<TungsteniteError> for Error {
    fn from(err: TungsteniteError) -> Error {
        Error::Tungstenite(err)
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Error {
        Error::Serenity(err)
    }
}

fn main() {
    tokio::run_async(
        async {
            if let Err(err) = kankyo::load() {
                eprintln!("Couldn't load .env file: {:?}", err);
            }

            env_logger::init();
            let token = kankyo::key("DISCORD_TOKEN").expect("Token not present in environment");
        },
    );
}
