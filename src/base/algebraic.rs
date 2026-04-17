//! # Complex numbers.
//! src/base/complex.rs

use std::{
	ops,
	fmt,
};
use float_cmp;

use crate::base::defaults::{self, Real};
use crate::base::common::{Number, Complex, ToComplex, Complexes};
use crate::base::polar;

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
	/// `real` part of the `Algebraic` **structure**.
	pub real: Real,
	/// `imaginary` part of the `Algebraic` **structure**.
	pub imaginary: Real
}

impl Algebraic {
	/// Instantiate a new `Algebraic` complex number.
	#[inline]
	pub fn new(real: Real, imaginary: Real) -> Algebraic {
		Algebraic {
			real,
			imaginary,
		}
	}

	/// Create a new conjugate complex.
	#[inline]
	pub fn conjugate(self: &Self) -> Algebraic {
		Algebraic { 
			real: self.real, 
			imaginary: -1.0 * self.imaginary 
		}
	}

	/// Compute the distance between `self` and `other`.
	#[inline]
	pub fn distance_to(self: &Self, other: Algebraic) -> Real {
		Real::sqrt(
			(other.real - self.real) * (other.real - self.real)
			+ (other.imaginary - self.imaginary) * (other.imaginary - self.imaginary)
		)
	}

	/// Compute the distance squared between `self` and `other`.
	#[inline]
	pub fn distance_to_squared(self: &Self, other: Algebraic) -> Real {
		(other.real - self.real) * (other.real - self.real)
		+ (other.imaginary - self.imaginary) * (other.imaginary - self.imaginary)
	}
}

impl<C> Number<C> for Algebraic 
where C: Complex
{}

impl Complex for Algebraic {
	const TYPE: Complexes = Complexes::ALGEBRAIC;

	#[inline]
	fn real(self: &Self) -> Real {
		self.real
	}

	#[inline]
	fn imaginary(self: &Self) -> Real {
		self.imaginary
	}

	#[inline]
	fn absolute(self: &Self) -> Real {
		Real::sqrt(self.real * self.real + self.imaginary * self.imaginary)
	}

	#[inline]
	fn absolute_squared(self: &Self) -> Real {
		self.real * self.real + self.imaginary * self.imaginary
	}

	#[inline]
	fn argument(self: &Self) -> Real {
		if float_cmp::approx_eq!(Real, self.absolute(), 0.0, ulps = defaults::ULPS) {
			Real::NAN
		} else {
			Real::acos(self.real / self.absolute()) * self.imaginary.signum()
		}
	}
	
	fn is_zero(self: &Self) -> bool {
		float_cmp::approx_eq!(Real, self.real, 0.0, ulps = defaults::ULPS) 
		&& float_cmp::approx_eq!(Real, self.imaginary, 0.0, ulps = defaults::ULPS) 
	}

	fn is_pure_real(self: &Self) -> bool {
		float_cmp::approx_eq!(Real, self.imaginary, 0.0, ulps = defaults::ULPS) 
	}

	fn is_pure_imaginary(self: &Self) -> bool {
		float_cmp::approx_eq!(Real, self.real, 0.0, ulps = defaults::ULPS) 
	}

	#[inline]
	fn factor(self: &Self, x: Real) -> Self {
		Algebraic::new(
			self.real() * x,
			self.imaginary() * x,
		)
	}

	fn are_opposed<C>(self: &Self, other: C) -> bool
	where C: Complex 
	{
		float_cmp::approx_eq!(Real, self.imaginary(), -other.imaginary())
		&& float_cmp::approx_eq!(Real, self.real(), -other.real())
	}
}

impl Default for Algebraic {
	#[inline]
	fn default() -> Self {
		Algebraic::new( 
			0 as Real, 
			0 as Real 
		)
	}
}

impl<C> PartialEq<C> for Algebraic 
where C: Complex
{
	/// Tests for self and other values to be equal, and is used by `==`.
	/// 
	/// Because of `Real`s being `float`, comparison is using `float_cmp`.
	fn eq(self: &Self, other: &C) -> bool {
		float_cmp::approx_eq!(Real, self.real(), other.real(), ulps = defaults::ULPS)
		&& float_cmp::approx_eq!(Real, self.imaginary(), other.imaginary(), ulps = defaults::ULPS)
	}
}

impl<C> ops::Add<C> for Algebraic 
where C: Complex
{
	type Output = Self;

	#[inline]
	fn add(self: Self, other: C) -> Self::Output {
		Algebraic::new(
			self.real() + other.real(),
			self.imaginary() + other.imaginary(),
		)
	}
}

impl<C> ops::Sub<C> for Algebraic 
where C: Complex
{
	type Output = Self;

	#[inline]
	fn sub(self: Self, other: C) -> Self::Output {
		Algebraic {
			real: self.real() - other.real(),
			imaginary: self.imaginary() - other.imaginary(),
		}
	}
}

impl<C> ops::Mul<C> for Algebraic 
where C: Complex
{
	type Output = Self;

	#[inline]
	fn mul(self: Self, other: C) -> Self::Output {
		Algebraic {
			real: self.real() * other.real() - self.imaginary() * other.imaginary(),
			imaginary: self.real() * other.imaginary() + other.real() * self.imaginary(),
		}
	}
}

impl<C> ops::Div<C> for Algebraic 
where C: Complex
{
	type Output = Self;

	#[inline]
	fn div(self: Self, other: C) -> Self::Output {
		if self.is_pure_imaginary() {
			Algebraic {
				real: 0.0,
				imaginary: self.imaginary() / other.imaginary(),
			}
		} else if self.is_pure_real() {
			Algebraic {
				real: self.real() / other.real(),
				imaginary: 0.0,
			}
		} else {
			let denominator: Real = other.real() * other.real() + other.imaginary() * other.imaginary();
			Algebraic {
				real: (self.real() * other.real() + self.imaginary() * other.imaginary()) / denominator,
				imaginary: (self.imaginary() * other.real() - self.real() * other.imaginary()) / denominator,
			}
		}
	}
}

impl ops::Neg for Algebraic {
	type Output = Self;

	#[inline]
	fn neg(self: Self) -> Self::Output {
		Algebraic {
			real: -self.real,
			imaginary: -self.imaginary,
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

impl ToComplex for Algebraic {
	/// Create an `Algebraic` instance from `Self`.
	/// 
	/// As `Self` is already `Algebraic`, **clones it**.
	#[inline]
	fn to_algebraic(self: &Self) -> self::Algebraic {
		self.clone()
	}

	/// Create a `Polar` instance from `Algebraic`.
	#[inline]
	fn to_polar(self: &Self) -> polar::Polar {
		polar::Polar { 
			theta: self.argument(), 
			distance: self.absolute(), 
	  	}
	}
}
