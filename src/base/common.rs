//! # Complex numbers.
//! src/base/common.rs
//! 
//! Common shared behavior across complex number forms.


use super::types::Real;

/// # `Shared` complex propriety.
/// Define the "must-have" for any form of complex number.
pub trait Shared {
	/// Absolute value of the complex number.
	/// 
	/// Return |z|, the module, the distance from 0, in other words.
	/// 
	/// Computed using a simple euclidean distance a² + b², or explicit with `Polar`. 
	fn absolute(self: &Self) -> Real;
	/// Get the `argument` theta **θ**, an angle in **radians**.
	/// 
	/// It is smallest directed angle from the _x+_ axis to `self`.
	fn argument(self: &Self) -> Real;
	/// Check if the complex number is zero. In different forms, for z:
	/// - `Algebraic`: 0 + 0i = 0;
	/// - `Polar`: |z| = 0;
	fn is_zero(self: &Self) -> bool;
}
