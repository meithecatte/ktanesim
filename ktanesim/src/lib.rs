#![feature(proc_macro_hygiene)]
#![feature(fnbox)]
pub mod modules;
pub mod prelude;

pub struct Bomb {
    pub rule_seed: u32,
}
