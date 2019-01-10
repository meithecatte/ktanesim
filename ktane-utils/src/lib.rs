#![feature(slice_sort_by_cached_key)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate ordered_float;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate enum_map;

pub mod edgework;
pub mod random;

pub use crate::random::RuleseedRandom;
pub use crate::edgework::{Edgework,SerialNumber,PortType,PortPlate,IndicatorCode,IndicatorState};
