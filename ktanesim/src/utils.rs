use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::builder::CreateMessage;

/// Try to send a message and handle any errors
pub fn send_message<F>(ctx: &Context, channel_id: ChannelId, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'b>) -> &'b mut CreateMessage<'b>,
{
    if let Err(why) = channel_id.send_message(&ctx.http, f) {
        error!("Couldn't send message: {:#?}", why);
    }
}
