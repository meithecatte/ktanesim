use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// A key for the [`ShareMap`] in the [`Context`], refers to a mapping from [`ChannelId`]s to
/// [`Bomb`]s.
pub struct Bombs;

impl TypeMapKey for Bombs {
    type Value = HashMap<ChannelId, BombRef>;
}

/// Helper function that, given a [`Context`] returns the [`Bomb`] for a given [`Message`]. If no
/// bomb is ticking in the message's channel, [`None`] is returned.
pub fn get_bomb(ctx: &Context, msg: &Message) -> Option<BombRef> {
    ctx.data
        .read()
        .get::<Bombs>()
        .unwrap()
        .get(&msg.channel_id)
        .map(Arc::clone)
}

/// Helper wrapper around [`get_bomb`] that returns an error message if there is no bomb.
pub fn need_bomb(ctx: &Context, msg: &Message) -> Result<BombRef, ErrorMessage> {
    get_bomb(ctx, msg).ok_or_else(no_bomb)
}

/// Helper function that, given a [`Context`] returns whether a bomb is ticking in the channel
/// corresponding to a [`Message`].
pub fn running_in(ctx: &Context, msg: &Message) -> bool {
    ctx.data
        .read()
        .get::<Bombs>()
        .unwrap()
        .contains_key(&msg.channel_id)
}

pub fn no_bomb() -> ErrorMessage {
    // TODO: link help
    (
        "No bomb in this channel".to_owned(),
        "No bomb is currently running in this channel. \
         Check out the help for more details."
            .to_owned(),
    )
}

pub fn end_bomb(
    ctx: &Context,
    bomb: &mut BombData,
    drop_callback: impl FnOnce(&mut BombData) + Send + Sync + 'static
) {
    ctx.data
        .write()
        .get_mut::<Bombs>()
        .unwrap()
        .remove(&bomb.channel)
        .unwrap();
    bomb.timer.freeze();
    bomb.drop_callback = Some(Box::new(drop_callback));
    update_presence(ctx);
}

pub fn update_presence(ctx: &Context) {
    let bomb_count = ctx.data.read().get::<Bombs>().unwrap().len();
    let status = if bomb_count == 0 {
        OnlineStatus::Idle
    } else {
        OnlineStatus::Online
    };
    ctx.set_presence(
        Some(Activity::playing(&format!(
            "{} bomb{}. !help for help",
            bomb_count,
            if bomb_count == 1 { "" } else { "s" },
        ))),
        status,
    );
}
