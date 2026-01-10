//! # Complex numbers.
//! src/base/complex.rs

use std::{
	ops,
	fmt,
};
use float_cmp;


use crate::base::types::Number;

/// # Complex `Algebraic` number. 
/// A generic type `T` and 2 arguments:
/// - `real`: `T`,
/// - `imaginary`: `T`.
/// 
/// `T` must be **numerical**.
/// 
/// `Algebraic` implements notable traits:
/// - `Clone`,
/// - `Copy`,
/// - `Add`,
/// - `Sub`
/// - `Mul`,
/// - `Div`,
/// - `Debug`,
/// - `Display`.
#[derive(Clone, Copy)]
pub struct Algebraic {
	pub real: Number,
	pub imaginary: Number
}

impl Algebraic {
	/// Instantiate a new `Algebraic` complex number.
	pub fn new(real: Number, imaginary: Number) -> Algebraic {
		Algebraic {
			real,
			imaginary,
		}
	}

	/// Create a new conjugate complex.
	pub fn conjugate(self: &Self) -> Algebraic {
		Algebraic { 
			real: self.real, 
			imaginary: (-1 as Number) * self.imaginary 
		}
	}

	/// Return the module, the absolute value, or the distance from 0.
	pub fn absolute(self: &Self) -> Number {
		Number::sqrt(self.real * self.real + self.imaginary * self.imaginary)
	}

	/// Get the `argument` _theta_, smallest directed angle from _x_ to `self`.  
	pub fn argument(self: &Self) -> Number {
		Number::acos(self.real / self.absolute()) * self.imaginary.signum()
	}

	/// Check if the complex number is 0 + 0i = 0.
	pub fn is_zero(self: &Self) -> bool {
		self.real == 0 as Number && self.imaginary == 0 as Number
	}
}

impl Default for Algebraic {
	fn default() -> Self {
		Algebraic { 
			real: 0 as Number, 
			imaginary: 0 as Number 
		}
	}
}

impl PartialEq for Algebraic {
	fn eq(self: &Self, other: &Self) -> bool {
		float_cmp::approx_eq!(Number, self.real, other.real, ulps = 2)
		&& float_cmp::approx_eq!(Number, self.imaginary, other.imaginary, ulps = 2)
	}
}

impl ops::Add for Algebraic {
	type Output = Self;

	fn add(self: Self, other: Self) -> Self::Output {
		Algebraic {
			real: self.real + other.real,
			imaginary: self.imaginary + other.imaginary,
		}
	}
}

impl ops::Sub for Algebraic {
	type Output = Self;

	fn sub(self: Self, other: Self) -> Self::Output {
		Algebraic {
			real: self.real - other.real,
			imaginary: self.imaginary - other.imaginary,
		}
	}
}

impl ops::Neg for Algebraic {
	type Output = Self;

	fn neg(self: Self) -> Self::Output {
		Algebraic {
			real: - self.real,
			imaginary: - self.imaginary,
		}
	}
}

impl ops::Mul for Algebraic {
	type Output = Self;

	fn mul(self: Self, other: Self) -> Self::Output {
		Algebraic {
			real: self.real * other.real - self.imaginary * other.imaginary,
			imaginary: self.real * other.imaginary + other.real * self.imaginary,
		}
	}
}

impl ops::Div for Algebraic {
	type Output = Self;

	fn div(self: Self, other: Self) -> Self::Output {
		let denominator: Number = other.real * other.real + other.imaginary * other.imaginary;
		Algebraic {
			real: (self.real * other.real + self.imaginary * other.imaginary) / denominator,
			imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denominator,
		}
	}
}

impl fmt::Debug for Algebraic {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.debug_struct("Algebraic")
			.field("real", &self.real)
			.field("imaginary", &self.imaginary)
			.finish()
	}
}

impl fmt::Display for Algebraic {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.imaginary.is_sign_positive() {
			write!(formatter, "{} + {}i", self.real, self.imaginary)
		} else {
			write!(formatter, "{} - {}i", self.real, self.imaginary.abs())
		}
	}
}