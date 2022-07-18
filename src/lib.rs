//! Rust and WASM OWL Library
//!
//! # Examples
//!
//! ```rust
//! use owlish::api::*;
//!
//!
//!
//! ```

pub mod api;
pub mod owl;

pub mod examples;
pub mod parser;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
