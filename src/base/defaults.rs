//! # Complex numbers.
//! src/base/types.rs

/// Define the current type in use.
/// 
/// Use primitive: `Real` is used in `as` casts.
pub type Real = f64;

/// ULPS level in `float_cmp` for the whole crate. 
pub(crate) const ULPS: i64 = 2;
