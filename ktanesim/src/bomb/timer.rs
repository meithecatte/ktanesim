use std::time::{Duration, Instant};
use std::fmt;

pub struct Timer {
    mode: TimerMode,
    strikes: u32,
    /// Last time the `display` fiel was updated.
    last_update: Instant,
    /// The display at the time of the `last_update`.
    display: Duration,
    frozen: bool,
}

use strum_macros::EnumString;
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimerMode {
    Normal { time: Duration, strikes: u32 },
    Zen,
    Time,
}

impl Timer {
    pub fn new(mode: TimerMode) -> Timer {
        let starting_time = match mode {
            TimerMode::Normal { time, .. } => time,
            TimerMode::Zen => Duration::from_secs(0),
            TimerMode::Time => Duration::from_secs(300),
        };

        Timer {
            mode,
            strikes: 0,
            last_update: Instant::now(),
            display: starting_time,
            frozen: false,
        }
    }

    pub fn mode(&self) -> TimerMode {
        self.mode
    }

    pub fn strikes(&self) -> u32 {
        self.strikes
    }

    pub fn get_display(&self) -> Duration {
        if self.frozen {
            self.display
        } else {
            self.get_display_for_time(Instant::now())
        }
    }

    fn get_display_for_time(&self, now: Instant) -> Duration {
        let real_delta = now.duration_since(self.last_update);
        let bomb_delta = self.to_bomb_time(real_delta);

        if self.going_forward() {
            self.display + bomb_delta
        } else {
            self.display.checked_sub(bomb_delta).unwrap_or_else(|| Duration::from_secs(0))
        }
    }

    /// Must be called before each change of speed.
    fn update(&mut self) {
        if !self.frozen {
            let now = Instant::now();
            self.display = self.get_display_for_time(now);
            self.last_update = now;
        }
    }

    fn to_bomb_time(&self, duration: Duration) -> Duration {
        match self.mode {
            TimerMode::Normal { .. } => match self.strikes {
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
            TimerMode::Normal { .. } => match self.strikes {
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
        match self.mode {
            TimerMode::Normal { .. } => true,
            _ => false,
        }
    }

    /// Register a strike on the bomb. Does not perform leaderboard accounting and does not notify
    /// the players. Returns `true` if the strike should cause an explosion.
    pub fn strike(&mut self) -> bool {
        if self.variable_speed() {
            self.update();
        }

        self.strikes += 1;

        match self.mode {
            TimerMode::Normal { strikes, .. } => self.strikes >= strikes,
            TimerMode::Zen => false,
            TimerMode::Time => unimplemented!(),
        }
    }

    pub fn freeze(&mut self) {
        self.update();
        self.frozen = true;
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.get_display();
        let mut seconds = value.as_secs();
        if seconds >= 3600 {
            let hours = seconds / 3600;
            seconds %= 3600;
            write!(f, "{:02}:", hours)?;
        }

        if seconds >= 60 {
            let minutes = seconds / 60;
            seconds %= 60;
            write!(f, "{:02}:{:02}", minutes, seconds)
        } else {
            write!(f, "{:02}.{:02}", seconds, value.subsec_millis() / 10)
        }
    }
}
