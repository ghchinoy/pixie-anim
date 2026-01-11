//! # Pixie-Anim
//!
//! A zero-dependency, SIMD-accelerated GIF optimizer.

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "simd", feature = "wasm")), forbid(unsafe_code))]

/// Bit-level reading and writing utilities.
pub mod bits;
/// Color space conversions and perceptual distance.
pub mod color;
/// Inter-frame Delta Compression.
pub mod delta;
/// Error handling.
pub mod error;
/// GIF89a Structure and Writing.
pub mod gif;
pub mod lzw;
pub mod quant;
pub mod simd;

/// High-level optimization engine.
#[cfg(feature = "image")]
pub mod engine;

#[cfg(feature = "cli")]
pub mod common;

/// Evaluation and Benchmarking utilities.
#[cfg(feature = "cli")]
pub mod evaluation;

/// WebAssembly bindings for Pixie-Anim.
#[cfg(feature = "wasm")]
pub mod wasm;

pub use error::{Error, Result};
