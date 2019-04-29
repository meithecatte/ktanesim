use crate::prelude::*;
use core::num::IntErrorKind; // wtf?
use joinery::prelude::*;
use serenity::builder::CreateMessage;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Arc;

/// Try to send a message and handle any errors
pub fn send_message<F>(http: &Arc<serenity::http::raw::Http>, channel_id: ChannelId, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'b>) -> &'b mut CreateMessage<'b>,
{
    if let Err(why) = channel_id.send_message(http, f) {
        error!("Couldn't send message: {:#?}", why);
    }
}

/// Return the URL of a `user`'s Discord avatar.
pub fn user_avatar(user: &User) -> String {
    user.avatar_url()
        .unwrap_or_else(|| user.default_avatar_url())
}

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
        Ok(n) if n <= max => Ok(n),
        Err(ref why) if *why.kind() != IntErrorKind::Overflow => Err(RangedIntError::Other),
        _ => Err(RangedIntError::TooLarge),
    }
}

/// Turn a parameters iterator into a message about trailing parameters if there are any.
pub fn trailing_parameters<'a>(mut params: Parameters<'a>) -> Option<impl std::fmt::Display + 'a> {
    if let Some(first) = params.next() {
        let params = std::iter::once(first)
            .chain(params)
            .join_with(joinery::separators::Space);
        Some(params)
    } else {
        None
    }
}
