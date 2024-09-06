#![feature(
    associated_type_defaults,
    let_chains,
    macro_metavar_expr,
)]

mod binding;
mod compose;
mod context;
mod event;
mod ffi;
mod node;
mod root;
mod state;
mod utils;

pub use binding::*;
pub use compose::*;
pub use context::*;
pub use event::*;
pub use ffi::*;
pub use node::*;
pub use root::*;
pub use state::*;
pub use utils::*;

// We alias the crate (nuit-core) to nuit to make nuit-derive's derive macros
// work in the same way as the do for nuit's library clients (which shouldn't
// need to know about nuit-core). See https://stackoverflow.com/a/57049687.
pub(crate) extern crate self as nuit;
