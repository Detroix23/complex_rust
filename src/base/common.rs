//! # Complex numbers.
//! src/base/common.rs
//! 
//! Common shared behavior across complex number forms.

use std::{ops, fmt};

use crate::base::defaults::{Real, PI};
use crate::base::{algebraic, polar};

/// `0`
pub const ZERO: algebraic::Algebraic = algebraic::Algebraic { real: 0.0, imaginary: 0.0 };
/// `0e^(0i)`
pub const ZERO_POLAR: polar::Polar = polar::Polar { distance: 0.0, theta: 0.0 };
/// `1`
pub const ONE: algebraic::Algebraic = algebraic::Algebraic { real: 1.0, imaginary: 0.0 };
/// `1e^0`
pub const ONE_POLAR: polar::Polar = polar::Polar { distance: 1.0, theta: 0.0 };
/// `0 + 1i`
pub const I: algebraic::Algebraic = algebraic::Algebraic { real: 0.0, imaginary: 1.0 };
/// `1e^(i*pi/2)`
pub const I_POLAR: polar::Polar = polar::Polar { distance: 1.0, theta: PI / 2.0 };

/// # Defines `Complexes` type.
pub enum Complexes {
	ALGEBRAIC,
	POLAR,
}

/// # Define any `Number`, real or complex.
/// It's purpose is describing, ordering. It registers:
/// - `Sized`;
/// - `Add`;
/// - `Mul`;
/// - `Sub`;
/// - `Div`;
/// - `PartialEq`;
pub trait Number<C>: 
	Sized
	+ Copy
 	+ ops::Add<C>
	+ ops::Mul<C> 
	+ ops::Sub<C> 
	+ ops::Div<C> 
	+ ops::Neg
	+ PartialEq 
where C: Complex
{}

/// # Shared `Complex` proprieties.
/// Define the "must-have" for any form of complex number.
/// 
/// It requires some methods, and registers some traits:
/// - `Sized`;
/// - `complex_rust::base::common::Number`;
/// - `Debug`;
/// - `Display`;
pub trait Complex:
	Sized
	+ Number<Self>
	+ ToComplex
	+ Default
	+ fmt::Debug
	+ fmt::Display
{
	/// Get `Complexes` sub-type.
	const TYPE: Complexes;

	/// Get the coefficient of the real part.
	fn real(self: &Self) -> Real;

	/// Get the coefficient of the imaginary part.
	fn imaginary(self: &Self) -> Real;

	/// Absolute value of the `Complex` number.
	/// 
	/// Return |z| >= 0, the module, the distance from 0, in other words.
	/// 
	/// Computed using a simple euclidean distance √(a² + b²), or explicit with `Polar`. 
	fn absolute(self: &Self) -> Real;

	/// Absolute value squared of the `Complex` number. Result not square rooted for faster comparisons.
	/// 
	/// Return |z|², the module, the distance from 0, in other words.
	/// 
	/// Computed using a simple euclidean distance a² + b², or explicit with `Polar`.
	fn absolute_squared(self: &Self) -> Real;

	/// Get the `argument` theta -π <= **θ** <= π, an angle in **radians**.
	/// 
	/// It is smallest directed angle from the _x+_ axis to `self`.
	fn argument(self: &Self) -> Real;
	
	/// Check if the `Complex` number is zero. In different forms, for z:
	/// - `Algebraic`: 0 + 0i = 0;
	/// - `Polar`: |z| = 0;
	fn is_zero(self: &Self) -> bool;

	/// Returns if only composed of a `real` part.
	fn is_pure_real(self: &Self) -> bool;

	/// Returns if only composed of an `imaginary` part.
	fn is_pure_imaginary(self: &Self) -> bool;

	/// Multiplies it`self` by a `Real` `x`.
	fn factor(self: &Self, x: Real) -> Self;

	/// Returns `true` if `self` - `other` = 0.
	fn are_opposed<C>(self: &Self, other: C) -> bool
	where C: Complex;
} 

/// # Parsing `ToComplex`s and between.
/// Allow transfers:
/// - `to_algebraic`;
/// - `to_polar`;
pub trait ToComplex {
	/// Create an `Algebraic` instance from `Self`.
	fn to_algebraic(self: &Self) -> algebraic::Algebraic;

	/// Create a `Polar` instance from `Self`.
	fn to_polar(self: &Self) -> polar::Polar;
}

/*
// Useless implementation of `Number` to the `Real`s.
impl<C> Number<C> for Real 
where C: Complex
{}

impl Complex for Real {
	fn real(self: &Self) -> Real {
		*self
	}

	fn imaginary(self: &Self) -> Real {
		0 as Real
	}

	fn absolute(self: &Self) -> Real {
		self.abs()
	}

	fn absolute_squared(self: &Self) -> Real {
		self.powi(2)
	}

	fn argument(self: &Self) -> Real {
		if *self >= 0 as Real {
			0 as Real
		} else {
			PI
		}
	}

	fn is_zero(self: &Self) -> bool {
		*self == 0 as Real
	}
}
*/
