use crate::prelude::*;
use core::num::IntErrorKind; // wtf?
use serenity::builder::CreateMessage;
use std::num::ParseIntError;
use std::str::FromStr;

/// Try to send a message and handle any errors
pub fn send_message<F>(ctx: &Context, channel_id: ChannelId, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'b>) -> &'b mut CreateMessage<'b>,
{
    if let Err(why) = channel_id.send_message(&ctx.http, f) {
        error!("Couldn't send message: {:#?}", why);
    }
}

pub type CommandResult = Result<(), ErrorMessage>;
pub type ErrorMessage = (String, String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangedIntError {
    TooLarge,
    Other,
}

pub fn ranged_int_parse<T>(input: &str, max: T) -> Result<T, RangedIntError>
where
    T: FromStr<Err = ParseIntError> + Ord + Copy,
{
    match input.parse() {
        Ok(n) if n < max => Ok(n),
        Err(ref why) if *why.kind() != IntErrorKind::Overflow => Err(RangedIntError::Other),
        _ => Err(RangedIntError::TooLarge),
    }
}
