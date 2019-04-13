//! Reexports everything needed in an average module.
pub use serenity::prelude::*;
pub use serenity::model::prelude::*;

pub use crate::bomb::Bomb;
pub use crate::modules::{
    module_canvas, output_png, EventResponse, Module, ModuleCategory, ModuleDescriptor,
    ModuleOrigin, Render, RenderType, SolveLight,
};
pub use crate::utils::*;

pub type Parameters<'a> = std::str::SplitWhitespace<'a>;

// Reexported because of function signatures
pub use serenity::model::id::UserId;
pub use std::sync::MutexGuard;
pub use typemap::ShareMap;
