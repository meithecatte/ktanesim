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
    adjusted_start: Instant,
    starting_time: Duration,
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
            adjusted_start: Instant::now(),
            starting_time,
        }
    }

    pub fn get_mode(&self) -> TimerMode {
        self.mode
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

    /// Returns the time that should be displayed on the timer
    pub fn get_display(&self) -> Duration {
        let elapsed = self.to_bomb_time(self.adjusted_start.elapsed());

        if self.going_forward() {
            elapsed
        } else {
            self.starting_time - elapsed
        }
    }

    /// Add a strike to the counter and adjust the `adjusted_start` field so that the value on the
    /// display doesn't change.
    pub fn strike(&mut self) {
        let now = Instant::now();
        let elapsed = self.to_bomb_time(now.duration_since(self.adjusted_start));
        self.strikes += 1;
        self.adjusted_start = now - self.to_real_time(elapsed);
    }
}
