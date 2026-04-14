//! # Complex numbers.
//! src/base/common.rs
//! 
//! Common shared behavior across complex number forms.

use std::{ops, fmt};

use crate::base::defaults::Real;
use crate::base::{algebraic, polar};

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
	/// Get the coefficient of the real part.
	fn real(self: &Self) -> Real;

	/// Get the coefficient of the imaginary part.
	fn imaginary(self: &Self) -> Real;

	/// Absolute value of the complex number.
	/// 
	/// Return |z|, the module, the distance from 0, in other words.
	/// 
	/// Computed using a simple euclidean distance √(a² + b²), or explicit with `Polar`. 
	fn absolute(self: &Self) -> Real;

	/// Absolute value squared of the complex number. Result not square rooted for faster comparisons.
	/// 
	/// Return |z|², the module, the distance from 0, in other words.
	/// 
	/// Computed using a simple euclidean distance a² + b², or explicit with `Polar`.
	fn absolute_squared(self: &Self) -> Real;

	/// Get the `argument` theta **θ**, an angle in **radians**.
	/// 
	/// It is smallest directed angle from the _x+_ axis to `self`.
	fn argument(self: &Self) -> Real;
	
	/// Check if the complex number is zero. In different forms, for z:
	/// - `Algebraic`: 0 + 0i = 0;
	/// - `Polar`: |z| = 0;
	fn is_zero(self: &Self) -> bool;

	/// Multiply it`self` by a `Real` `x`.
	fn real_multiplication(self: &Self, x: Real) -> Self;
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
			std::f64::consts::PI
		}
	}

	fn is_zero(self: &Self) -> bool {
		*self == 0 as Real
	}
}
*/
