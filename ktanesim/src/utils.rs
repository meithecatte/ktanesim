use crate::prelude::*;
use core::num::IntErrorKind; // wtf?
use joinery::prelude::*;
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

/// Parse an integer. If the `input` is larger than `max`, a [`TooLarge`][RangedIntError::TooLarge]
/// error is returned. This is the case even if the result wouldn't fit in the number type.
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

/// Turn a parameters iterator into a message about trailing parameters if there are any.
pub fn trailing_parameters(
    mut params: Parameters<'_>,
    f: impl FnOnce(&dyn std::fmt::Display) -> ErrorMessage,
) -> CommandResult {
    if let Some(first) = params.next() {
        let params = std::iter::once(first)
            .chain(params)
            .join_with(joinery::separators::Space);
        Err(f(&params))
    } else {
        Ok(())
    }
}
