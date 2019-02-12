#![feature(async_await, await_macro)]
#![feature(proc_macro_hygiene)]
#![feature(fnbox)]
pub mod backoff;
pub mod bomb;
pub mod modules;
pub mod prelude;
#[macro_use]
pub mod util_macros;

#[macro_use]
extern crate log;

pub use bomb::Bomb;
