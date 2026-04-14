//! # Complex numbers.
//! src/base/polar.rs

use std::{fmt, ops};

use float_cmp;

use crate::base::common::{Number, Complex, ToComplex}; 
use crate::base::defaults::{self, Real};
use crate::base::algebraic;

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
	#[inline]
	pub fn new(theta: Real, distance: Real) -> Polar {
		Polar { 
			theta, 
			distance 
		}
	}

	/// Returns a `String` formatted the trigonometric way.
	/// 
	/// _Example_: 
	/// ```rust
	/// let z1 = complex::Polar { theta: 1.23, distance: 6.0 };
	/// 
	/// assert_eq!(z1.trigonometric(), "6(cos(1.23) + isin(1.23))")
	/// ```
	#[inline]
	pub fn trigonometric(self: &Self) -> String {
		format!("{}(cos({}) + isin({}))", self.distance, self.theta, self.theta)
	}

	/// Returns a `String` formatted the exponential way.
	/// 
	/// ```rust
	/// let z1 = complex::Polar { theta: 1.23, distance: 6.0 };
	/// 
	/// assert_eq!(z1.exponential(), "6e ^ (1.23i)")
	/// ```
	#[inline]
	pub fn exponential(self: &Self) -> String {
		format!("{}e ^ ({}i)", self.distance, self.theta)
	}
}

impl<C> Number<C> for Polar 
where C: Complex
{}

impl Complex for Polar {
	#[inline]
	fn real(self: &Self) -> Real {
		self.distance * self.theta.cos()
	}

	#[inline]
	fn imaginary(self: &Self) -> Real {
		self.distance * self.theta.sin()
	}

	#[inline]
	fn absolute(self: &Self) -> Real {
		self.distance
	}

	#[inline]
	fn absolute_squared(self: &Self) -> Real {
		self.distance * self.distance
	}

	#[inline]
	fn argument(self: &Self) -> Real {
		self.theta
	}

	fn is_zero(self: &Self) -> bool {
		float_cmp::approx_eq!(Real, self.distance, 0.0, ulps = defaults::ULPS)
	}

	#[inline]
	fn real_multiplication(self: &Self, x: Real) -> Self {
		Polar::new(
			self.argument(),
			self.absolute() * x,
		)
	}
}

impl Default for Polar {
	fn default() -> Self {
		Polar { theta: 0 as Real, distance: 0 as Real }
	}
}

impl PartialEq for Polar {
	/// Tests for self and other values to be equal, and is used by `==`.
	/// 
	/// Because of `Real`s being `float`, comparison is using `float_cmp`.
	fn eq(self: &Self, other: &Self) -> bool {
		float_cmp::approx_eq!(Real, self.distance, other.distance, ulps = defaults::ULPS)
		&& float_cmp::approx_eq!(Real, self.theta, other.theta, ulps = defaults::ULPS)
	}
}

impl<C> ops::Add<C> for Polar 
where C: Complex,
{
	type Output = Self;
	
	/// Perform the `+` operation between this `Polar` and a `Complex`.
	/// 
	/// As `Polar` additions, as is, are not practical, we do an `Algebraic` addition. 
	fn add(self: Self, other: C) -> Self::Output {
		algebraic::Algebraic::new(
			self.real() + other.real(),
			self.imaginary() + other.imaginary()
		).to_polar()
	}
}


impl<C> ops::Sub<C> for Polar 
where C: Complex
{
	type Output = Self;

	fn sub(self: Self, other: C) -> Self::Output {
		algebraic::Algebraic::new(
			self.real() + other.real(),
			self.imaginary() + other.imaginary()
		).to_polar()
	}
}

impl<C> ops::Mul<C> for Polar 
where C: Complex
{
	type Output = Self;

	/// Perform the `*` operation between this `Polar` and a `Complex`.
	/// 
	/// With:
	/// ```maths, ignore
	/// z1 = r1 * e ^ (θ1 * i),
	/// z2 = r2 * e ^ (θ2 * i)
	/// ```
	/// We have:
	/// ```maths, ignore
	/// z1 * z2 = z1 * z2 * e ^ i(θ1 + θ2)
	/// ```
	fn mul(self: Self, other: C) -> Self::Output {
		Polar {
			distance: self.absolute() * other.absolute(),
			theta: self.argument() + other.argument(),
		}
	}
}

impl<C> ops::Div<C> for Polar 
where C: Complex
{
	type Output = Self;

	/// Perform the `*` operation between this `Polar` and a `Complex`.
	/// 
	/// With 
	/// ```maths, ignore
	/// z1 = r1 * e ^ (θ1 * i),
	/// z2 = r2 * e ^ (θ2 * i)
	/// ```
	/// We have:
	/// ```maths, ignore
	/// z1 * z2 = z1 * z2 * e ^ i(θ1 + θ2)
	/// ```
	fn div(self: Self, other: C) -> Self::Output {
		Polar {
			distance: self.absolute() / other.absolute(),
			theta: self.argument() - other.argument(),
		}
	}
}

impl ops::Neg for Polar {
	type Output = Self;

	fn neg(self: Self) -> Self::Output {
		Polar {
			distance: -self.distance,
			theta: self.theta,
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

impl fmt::Display for Polar {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", self.exponential())
	}
}

impl ToComplex for Polar {
	#[inline]
	fn to_algebraic(self: &Self) -> algebraic::Algebraic {
		algebraic::Algebraic {
			real: self.distance * self.theta.cos(),
			imaginary: self.distance * self.theta.sin(),
		}
	}

	/// Create a `Polar` instance from `Self`.
	/// 
	/// As `Self` is already `Algebraic`, **clones it**.
	#[inline]
	fn to_polar(self: &Self) -> self::Polar {
		self.clone()
	}
}
