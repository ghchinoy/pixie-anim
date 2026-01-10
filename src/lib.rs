//! # Pixie-Anim
//!
//! A zero-dependency, SIMD-accelerated GIF optimizer.

#![cfg_attr(not(any(feature = "simd", feature = "wasm")), forbid(unsafe_code))]

pub mod bits;
pub mod color;
pub mod delta;
pub mod error;
pub mod gif;
pub mod lzw;
pub mod quant;
pub mod simd;

#[cfg(feature = "cli")]
pub mod common;

#[cfg(feature = "cli")]
pub mod evaluation;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use error::{Error, Result};