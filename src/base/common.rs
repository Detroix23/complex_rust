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
pub trait Number: 
	Sized
 	+ ops::Add 
	+ ops::Mul 
	+ ops::Sub 
	+ ops::Div 
	+ ops::Neg
	+ PartialEq 
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
	+ Number
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
} 

/// # Parsing `ToComplex`s and between.
/// Allow transfers:
/// - `to_algebraic`;
/// - `to_polar`;
pub trait ToComplex: Complex {
	/// Create an `Algebraic` instance from `Self`.
	fn to_algebraic(self: &Self) -> algebraic::Algebraic;

	/// Create a `Polar` instance from `Self`.
	fn to_polar(self: &Self) -> polar::Polar;
}
