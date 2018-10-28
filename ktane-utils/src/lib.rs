#![feature(slice_sort_by_cached_key)]

#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate ordered_float;

pub mod edgework;
pub mod random;

pub use crate::random::RuleseedRandom;
