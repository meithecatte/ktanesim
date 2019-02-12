use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

/// A utility implementation of exponential backoff. Stores the count of failed attempts so far.
#[derive(Clone, Copy, Debug)]
pub struct Backoff(u32);

impl Backoff {
    pub fn new() -> Self {
        Backoff(0)
    }

    /// Should be awaited before making an attempt
    pub fn delay(self) -> impl Future<Item = (), Error = tokio::timer::Error> {
        if self.0 == 0 {
            future::Either::A(future::ok(()))
        } else {
            let delay_sec = 2u64.saturating_pow(self.0 - 1).min(30);
            trace!("Reattempting in {} seconds", delay_sec);
            future::Either::B(Delay::new(Instant::now() + Duration::from_secs(delay_sec)))
        }
    }

    /// Record a success
    pub fn success(&mut self) {
        self.0 = 0
    }

    /// Record a failure
    pub fn failure(&mut self) {
        self.0 = self.0.saturating_add(1)
    }
}
