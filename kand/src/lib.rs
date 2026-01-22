//! # Kand
//!
//! A high-performance technical analysis library for Rust, inspired by TA-Lib.
//!
//! ## Overview
//!
//! Kand provides a comprehensive suite of technical analysis tools for financial market data analysis.
//! Built with Rust's safety and performance in mind, it offers a modern alternative to traditional
//! technical analysis libraries.
//!
//! ## Features
//!
//! - **High Performance**: Written in pure Rust with zero dependencies on external C libraries
//! - **Type Safety**: Leverages Rust's type system to prevent common errors at compile time
//! - **Flexible Types**: Supports both standard (32-bit) and extended (64-bit) precision modes
//! - **Comprehensive Indicators**: Implements popular technical indicators including:
//!   - Moving Averages (SMA, EMA, WMA)
//!   - Momentum Indicators (RSI, MACD)
//!   - Volume Indicators (OBV)
//!   - And more...
//!
//! ## Quick Start
//!
//! ```rust
//! use kand::ohlcv::sma;
//!
//! // Input price data
//! let prices = vec![2.0, 4.0, 6.0, 8.0, 10.0];
//! let period = 3;
//! let mut sma_values = vec![0.0; prices.len()];
//!
//! // Calculate SMA
//! sma::sma(&prices, period, &mut sma_values).unwrap();
//! // First (period-1) values will be NaN, then: [NaN, NaN, 4.0, 6.0, 8.0]
//!
//! // Calculate next SMA value incrementally
//! let prev_sma = 8.0; // Last SMA value
//! let new_price = 12.0; // New price to include
//! let old_price = 6.0; // Oldest price to remove
//!
//! let next_sma = sma::sma_inc(prev_sma, new_price, old_price, period).unwrap();
//! // next_sma = 10.0 ((8.0 + 10.0 + 12.0) / 3)
//! ```
//!
//! ## Feature Flags
//!
//! Kand can be configured through feature flags:
//!
//! ### Precision Modes
//! - `default = ["extended", "check"]`: 64-bit precision with basic validation checks
//! - `standard = ["f32", "i32"]`: Standard precision mode using 32-bit types
//! - `extended = ["f64", "i64"]`: Extended precision mode using 64-bit types
//!
//! ### Type Selection
//! - `f32`: Use 32-bit floating point numbers
//! - `f64`: Use 64-bit floating point numbers
//! - `i32`: Use 32-bit integers
//! - `i64`: Use 64-bit integers
//!
//! ### Validation
//! - `check`: Enable basic validation checks
//! - `check-nan = ["check"]`: Enable extended validation (includes basic checks)
//!
//! ## Safety and Error Handling
//!
//! All functions in Kand return a `Result` type, properly handling edge cases and
//! invalid inputs. Common error cases include:
//!
//! - `InvalidParameter`: Invalid input parameters (e.g., period < 2)
//! - `InvalidData`: Empty or invalid input data
//! - `LengthMismatch`: Input and output slice lengths don't match
//! - `InsufficientData`: Not enough data points for calculation
//! - `NaNDetected`: NaN values in input data (with `check-nan` feature)
//!
//! ## Performance Considerations
//!
//! The library is optimized for both speed and memory usage:
//!
//! - Incremental calculation support for real-time updates
//! - Configurable precision modes (standard/extended)
//! - In-place calculations to minimize memory allocations
//! - Optional validation checks that can be disabled for maximum performance
#![allow(clippy::similar_names, clippy::too_many_lines)]

pub mod ta;
pub use ta::*;

pub mod helper;

pub mod error;
pub use error::{KandError, Result};

/// Default floating-point precision type used across the library.
///
/// This type is determined by the enabled features:
/// - With feature "f64": Uses f64 (recommended for most cases)
/// - With feature "f32": Uses f32 (for memory-constrained environments)
/// - With no features enabled: Defaults to f64
///
/// The default configuration (feature "extended") provides f64 for:
/// - Higher precision calculations (15-17 decimal digits)
/// - Better handling of large price values
/// - More accurate technical indicator results
#[cfg(all(feature = "f32", not(feature = "f64")))]
pub type TAFloat = f32;

#[cfg(not(all(feature = "f32", not(feature = "f64"))))]
pub type TAFloat = f64; // Default to f64 when no features are enabled

/// Default integer type used for indicator outputs.
///
/// This type is determined by the enabled features:
/// - With feature "i64": Uses i64 (recommended for most cases)
/// - With feature "i32": Uses i32 (for memory-constrained environments)
/// - With no features enabled: Defaults to i32
///
/// The default configuration (feature "extended") provides i64 for:
/// - Larger value range (-2^63 to 2^63-1)
/// - Better precision in accumulation operations
/// - Future-proof for high-frequency data analysis
#[cfg(all(feature = "i32", not(feature = "i64")))]
pub type TAInt = i32;

#[cfg(not(all(feature = "i32", not(feature = "i64"))))]
pub type TAInt = i64; // Default to i64 when no features are enabled

/// Default type for indicator periods.
///
/// This is consistently `usize` across the library to represent time periods,
/// lookback windows, and other count-based parameters.
pub type TAPeriod = usize;

/// Global EPSILON value used for floating-point comparisons
/// to account for rounding errors in calculations.
///
/// This value is automatically set based on the floating-point type:
/// - f32: Uses f32::EPSILON (≈ 1.19e-7)
/// - f64: Uses f64::EPSILON (≈ 2.22e-16)
#[cfg(all(feature = "f32", not(feature = "f64")))]
pub const EPSILON: TAFloat = f32::EPSILON;

#[cfg(not(all(feature = "f32", not(feature = "f64"))))]
pub const EPSILON: TAFloat = f64::EPSILON;
