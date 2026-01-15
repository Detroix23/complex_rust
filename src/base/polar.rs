//! # Complex numbers.
//! src/base/polar.rs

use std::{
	fmt,
	ops,
};

use crate::base::{
	types::Real,
	common::Shared,
	algebraic,
};

/// # `Polar` complex number form.
/// Has 2 arguments:
/// - `theta`: angle in radians from the x+ axis.
/// - `distance`: from 0.
/// 
/// Then, represents both:
/// - `distance * (cos(theta) + i * sin(theta))`;
/// - `distance * exp(i * theta)`
#[derive(Clone, Copy)]
pub struct Polar {
	/// `theta`: `Real`, angle **θ**.
	pub theta: Real,
	pub distance: Real,
}

impl Polar {
	/// Instantiate a new `Polar` complex.
	pub fn new(theta: Real, distance: Real) -> Polar {
		Polar { 
			theta, 
			distance 
		}
	}

	/// Returns a `String` formatted the trigonometric way.
	/// 
	/// _Ex:_ `6(cos(1.23) + isin(1.23)`
	pub fn trigonometric(self: &Self) -> String {
		format!("{}(cos({}) + isin({})", self.distance, self.theta, self.theta)
	}

	/// Returns a `String` formatted the exponential way.
	/// 
	/// _Ex:_ `6e ^ (1.23i)`
	pub fn exponential(self: &Self) -> String {
		format!("{}e ^ ({}i)", self.distance, self.theta)
	}

	/// Convert a `Polar` to a new `Algebraic`.
	pub fn to_algebraic(self: &Self) -> algebraic::Algebraic {
		algebraic::Algebraic {
			real: self.distance * self.theta.cos(),
			imaginary: self.distance * self.theta.sin(),
		}
	}

	/// Raise `self` to the power of `exponent`, using exponential proprieties.
	/// 
	/// (r * e ^ (θi)) ^ n = r ^ n * e ^ (nθi)
	/// 
	/// Creates and returns a new `Polar` instance.
	pub fn power(self: &Self, exponent: i32) -> Polar {
		Polar { 
			theta: self.theta * exponent as Real, 
			distance: self.distance.powi(exponent),
		}
	}
}

impl Shared for Polar {
	fn absolute(self: &Self) -> Real {
		self.distance
	}

	fn argument(self: &Self) -> Real {
		self.theta
	}

	fn is_zero(self: &Self) -> bool {
		self.distance == 0.0
	}
}

impl ops::Mul for Polar {
	type Output = Self;

	/// Perform the `*` operation between 2 `Polar`s.
	/// 
	/// With z1 = r1 * e ^ (θ1 * i),
	/// z2 = r2 * e ^ (θ2 * i)
	/// 
	/// z1 * z2 = z1 * z2 * e ^ i(θ1 + θ2)
	fn mul(self: Self, other: Self) -> Self::Output {
		Polar {
			distance: self.distance * other.distance,
			theta: self.theta + other.theta,
		}
	}
}

impl ops::Div for Polar {
	type Output = Self;

	/// Perform the `*` operation between 2 `Polar`s.
	/// 
	/// With z1 = r1 * e ^ (θ1 * i),
	/// z2 = r2 * e ^ (θ2 * i)
	/// 
	/// z1 * z2 = z1 * z2 * e ^ i(θ1 + θ2)
	fn div(self: Self, other: Self) -> Self::Output {
		Polar {
			distance: self.distance / other.distance,
			theta: self.theta - other.theta,
		}
	}
}


impl fmt::Debug for Polar {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.debug_struct("Polar")
			.field("theta", &self.theta)
			.field("distance", &self.distance)
			.finish()
	}
}
