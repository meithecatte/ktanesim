#![feature(slice_sort_by_cached_key)]
#![feature(range_contains)]
#![feature(stmt_expr_attributes)]

// Temporary, EnumFlags needs a fix
#![allow(proc_macro_derive_resolution_fallback)]

pub mod edgework;
pub mod random;
pub mod modules {
    pub mod wires;
}
