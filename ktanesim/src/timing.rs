//! Handles the various timeouts used by the bot.
use crate::prelude::*;
use std::time::Duration;
use timer_heap::{TimerHeap, TimerType};
use parking_lot::Condvar;

pub struct TimingHandle {
    events: Mutex<TimerHeap<TimingEvent>>,
    condvar: Condvar,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TimingEvent {
    Explode(ChannelId),
}

impl TimingEvent {
    fn handle(self, handler: &Handler) {
        use TimingEvent::*;
        match self {
            Explode(channel) => {
                if let Some(bomb) = handler.bombs.read().get(&channel) {
                    //bomb.write().explode(
                } else {
                    warn!("no bomb to explode");
                }
            }
        }
    }
}

impl TimingHandle {
    pub fn new() -> Self {
        TimingHandle {
            events: Mutex::new(TimerHeap::new()),
            condvar: Condvar::new(),
        }
    }

    // Started on another thread by main.
    pub fn processing_loop(&self, handler: &'static Handler, pool: threadpool::ThreadPool) {
        let mut events = self.events.lock();
        loop {
            for event in events.expired() {
                debug!("Handling event {:?}", event);
                pool.execute(move || event.handle(handler));
            }

            if let Some(time_next) = events.time_remaining() {
                trace!("Calling wait_for");
                self.condvar.wait_for(&mut events, time_next);
            } else {
                trace!("Calling wait");
                self.condvar.wait(&mut events);
            }
        }
    }

    fn do_heap<T>(&self, f: impl FnOnce(&mut TimerHeap<TimingEvent>) -> T) -> T {
        let mut events = self.events.lock();
        let result = f(&mut events);
        self.condvar.notify_one();
        result
    }

    /// Insert a timer into the queue. Returns an Error if the `event` is already in the queue.
    pub fn insert(&self, event: TimingEvent, duration: Duration) -> Result<(), timer_heap::Error> {
        self.do_heap(|events| events.insert(event, duration, TimerType::Oneshot))
    }

    /// Insert a timer into the queue, replacing any existing timer with the same ID.
    ///
    /// Returns whether the timer was already in the queue.
    pub fn upsert(&self, event: TimingEvent, duration: Duration) -> bool {
        self.do_heap(|events| events.upsert(event, duration, TimerType::Oneshot))
    }

    /// Remove a timer from the queue. Returns `true` if it was in the queue, `false` otherwise.
    pub fn remove(&self, event: TimingEvent) -> bool {
        self.do_heap(|events| events.remove(event))
    }
}
