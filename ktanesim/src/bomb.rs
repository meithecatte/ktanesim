use crate::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use smallbitvec::SmallBitVec;
use std::collections::HashMap;
use std::sync::Arc;

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

pub struct Bomb {
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

use std::time::{Duration, Instant};

pub struct Timer {
    mode: TimerMode,
    strikes: u32,
    /// Last time the `display` fiel was updated.
    last_update: Instant,
    /// The display at the time of the `last_update`.
    display: Duration,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimerMode {
    Normal,
    Zen,
    Time,
}

impl Timer {
    pub fn new(mode: TimerMode, starting_time: Duration) -> Timer {
        Timer {
            mode,
            strikes: 0,
            last_update: Instant::now(),
            display: starting_time,
        }
    }

    pub fn get_mode(&self) -> TimerMode {
        self.mode
    }

    pub fn get_display(&self) -> Duration {
        self.get_display_for_time(Instant::now())
    }

    fn get_display_for_time(&self, now: Instant) -> Duration {
        let real_delta = now.duration_since(self.last_update);
        let bomb_delta = self.to_bomb_time(real_delta);

        if self.going_forward() {
            self.display + bomb_delta
        } else {
            self.display - bomb_delta
        }
    }

    /// Must be called before each change of speed.
    fn update(&mut self) {
        let now = Instant::now();
        self.display = self.get_display_for_time(now);
        self.last_update = now;
    }

    fn to_bomb_time(&self, duration: Duration) -> Duration {
        match self.mode {
            TimerMode::Normal => match self.strikes {
                0 => duration,
                1 => duration * 5 / 4,
                2 => duration * 3 / 2,
                _ => duration * 2,
            },
            _ => duration,
        }
    }

    fn to_real_time(&self, duration: Duration) -> Duration {
        match self.mode {
            TimerMode::Normal => match self.strikes {
                0 => duration,
                1 => duration * 4 / 5,
                2 => duration * 2 / 3,
                _ => duration / 2,
            },
            _ => duration,
        }
    }

    pub fn going_forward(&self) -> bool {
        self.mode == TimerMode::Zen
    }

    pub fn variable_speed(&self) -> bool {
        self.mode == TimerMode::Normal
    }

    /// Register a strike on the bomb. Does not perform leaderboard accounting and does not notify
    /// the players.
    pub fn strike(&mut self) {
        if self.variable_speed() {
            self.update();
        }

        self.strikes += 1;
    }
}
