#![feature(proc_macro_hygiene)]
#![feature(fnbox)]
pub mod modules;
pub mod prelude;

use prelude::*;
use smallbitvec::SmallBitVec;

pub struct Bomb {
    pub rule_seed: u32,
    pub timer: Timer,
    pub modules: Vec<Box<dyn Module>>,
    solve_state: SmallBitVec,
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
