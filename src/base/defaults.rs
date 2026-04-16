//! # Complex numbers.
//! src/base/types.rs

use std::f64;

/// Define the current type in use.
/// 
/// Use primitive: `Real` is used in `as` casts.
pub type Real = f64;

pub const PI: Real = std::f64::consts::PI;

/// ULPS level in `float_cmp` for the whole crate. 
pub(crate) const ULPS: i64 = 2;
