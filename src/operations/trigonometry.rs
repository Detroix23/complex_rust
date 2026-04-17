//! # Complex numbers.
//! src/operations/trigonometry.rs
//! 
//! Traits for `Trigonometry`.

use crate::{I, Algebraic, Polar};
use crate::operations::Exponentiation;

/// # Implement functions of `Trigonometry`.
/// Thanks to Euler's formulas.
pub trait Trigonometry {
	type SineResult;
	type CosineResult;
	type TangentResult;

	/// Compute the sine of `self`.
	/// 
	/// Using Euler's formula:
	/// ```maths, ignore
	/// sin(x) = (e^ix - e^-ix) / 2i
	/// ```
	fn sin(self: &Self) -> Self::SineResult;

	/// Compute the cosine of `self`.
	/// 
	/// Using Euler's formula:
	/// ```maths, ignore
	/// cos(x) = (e^ix + e^-ix) / 2
	/// ```
	fn cos(self: &Self) -> Self::CosineResult;

	/// Compute the tangent of `self`.
	/// 
	/// Using the simple definition:
	/// ```maths, ignore
	/// tan(x) = sin(x) / cos(x)
	/// ```
	fn tan(self: &Self) -> Self::TangentResult;
}

/// # Implement trigonometric `Hyperbolic` functions.
/// Thanks to Euler's formulas.
pub trait Hyperbolic {
	type SineResult;
	type CosineResult;
	type TangentResult;

	/// Compute the hyperbolic sine of `self`.
	/// 
	/// Using definition:
	/// ```maths, ignore
	/// sinh(x) = (e^x - e^-x) / 2
	/// ```
	fn sinh(self: &Self) -> Self::SineResult;

	/// Compute the hyperbolic cosine of `self`.
	/// 
	/// Using definition:
	/// ```maths, ignore
	/// cosh(x) = (e^x + e^-x) / 2
	/// ```
	fn cosh(self: &Self) -> Self::CosineResult;

	/// Compute the hyperbolic tangent of `self`.
	/// 
	/// Using definition:
	/// ```maths, ignore
	/// tanh(x) = sinh(x) / cosh(x)
	/// ```
	fn tanh(self: &Self) -> Self::TangentResult;
}



// ===========
// Algebraic
// ===========

impl Trigonometry for Algebraic {
	type SineResult = Polar;
	type CosineResult = Polar;
	type TangentResult = Polar;

	fn sin(self: &Self) -> Self::SineResult {
		let z_i: Algebraic = *self * I;
		(z_i.to_exp() - (-z_i).to_exp()) / Algebraic::new(0.0, 2.0)
	}

	fn cos(self: &Self) -> Self::CosineResult {
		let z_i: Algebraic = *self * I;
		(z_i.to_exp() + (-z_i).to_exp()) / Algebraic::new(2.0, 0.0)
	}

	fn tan(self: &Self) -> Self::TangentResult {
		Self::sin(self) / Self::cos(self)
	}
}

impl Hyperbolic for Algebraic {
	type SineResult = Polar;
	type CosineResult = Polar;
	type TangentResult = Polar;

	fn sinh(self: &Self) -> Self::SineResult {
		(self.to_exp() - (-*self).to_exp()) / Algebraic::new(2.0, 0.0)
	}

	fn cosh(self: &Self) -> Self::CosineResult {
		(self.to_exp() + (-*self).to_exp()) / Algebraic::new(2.0, 0.0)
	}

	fn tanh(self: &Self) -> Self::TangentResult {
		Self::sinh(self) / Self::cosh(self)
	}
}

// ===========
// Polar
// ===========

impl Trigonometry for Polar {
	type SineResult = Polar;
	type CosineResult = Polar;
	type TangentResult = Polar;

	fn sin(self: &Self) -> Self::SineResult {
		let z_i: Polar = *self * I;
		(z_i.to_exp() - (-z_i).to_exp()) / Algebraic::new(0.0, 2.0)
	}

	fn cos(self: &Self) -> Self::CosineResult {
		let z_i: Polar = *self * I;
		(z_i.to_exp() + (-z_i).to_exp()) / Algebraic::new(2.0, 0.0)
	}

	fn tan(self: &Self) -> Self::TangentResult {
		Self::sin(self) / Self::cos(self)
	}
}


impl Hyperbolic for Polar {
	type SineResult = Polar;
	type CosineResult = Polar;
	type TangentResult = Polar;

	fn sinh(self: &Self) -> Self::SineResult {
		(self.to_exp() - (-*self).to_exp()) / Algebraic::new(0.0, 2.0)
	}

	fn cosh(self: &Self) -> Self::CosineResult {
		(self.to_exp() + (-*self).to_exp()) / Algebraic::new(2.0, 0.0)
	}

	fn tanh(self: &Self) -> Self::TangentResult {
		Self::sinh(self) / Self::cosh(self)
	}
}
