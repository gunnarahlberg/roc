#![warn(clippy::all, clippy::dbg_macro)]
// See github.com/rtfeldman/roc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant)]

pub use roc_can::expr::Expr::*;

pub mod builtins;
pub mod sharing;
