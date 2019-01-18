#![feature(slice_sort_by_cached_key)]
#![feature(range_contains)]

extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;
#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate num_derive;
extern crate num_traits;
#[macro_use]
extern crate lazy_static;
extern crate ordered_float;
extern crate ordinal;
#[macro_use]
extern crate smallvec;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod edgework;
pub mod random;
pub mod modules {
    pub mod wires;
}
