use crate::prelude::*;
use ktane_utils::edgework::Edgework;
use smallbitvec::SmallBitVec;
use std::collections::HashMap;
use std::sync::Arc;

mod starting;
mod timer;
pub use starting::cmd_run;
pub use timer::{Timer, TimerMode};

/// A key for the [`ShareMap`] in the [`Context`], refers to a mapping from [`ChannelId`]s to
/// [`Bomb`]s.
pub struct Bombs;

impl TypeMapKey for Bombs {
    type Value = HashMap<ChannelId, Arc<RwLock<Bomb>>>;
}

/// Helper function that, given a [`Context`] returns the [`Bomb`] for a given [`Message`]. If no
/// bomb is ticking in the message's channel, [`None`] is returned.
pub fn get_bomb(ctx: &Context, msg: &Message) -> Option<Arc<RwLock<Bomb>>> {
    ctx.data
        .read()
        .get::<Bombs>()
        .and_then(|bombs| bombs.get(&msg.channel_id))
        .map(Arc::clone)
}

/// Helper function that, given a [`Context`] returns whether a bomb is ticking in the channel
/// corresponding to a [`Message`].
pub fn running_in(ctx: &Context, msg: &Message) -> bool {
    ctx.data
        .read()
        .get::<Bombs>()
        .map_or(false, |bombs| bombs.contains_key(&msg.channel_id))
}

pub struct Bomb {
    pub edgework: Edgework,
    pub rule_seed: u32,
    pub timer: Timer,
    pub modules: Vec<Box<dyn Module>>,
    solve_state: SmallBitVec,
    pub defusers: HashMap<UserId, Defuser>,
}

/// Stores information about a user that has interacted with the current bomb.
pub struct Defuser {
    /// The module number of the last module viewed. Zero-indexed.
    pub last_view: Option<u32>,
}
