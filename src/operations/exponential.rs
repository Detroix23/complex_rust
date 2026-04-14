//! # Complex numbers.
//! src/operations/exponential.rs

use crate::{Algebraic, Polar, Complex, Real, ToComplex};

/// # Implement `Exponentiation` for numbers.
/// ```maths, ignore
/// e ^ {x}
/// ```
pub trait Exponentiation {
	/// Define the return type of the method `to_exp`.
	type Result;

	/// Raise `self` to the exponent of `e` (`e`: Euler's number).
	/// ```maths, ignore
	/// e ^ self
	/// ```
	/// Returns a `Polar`.
	fn to_exp(self: &Self) -> Self::Result;
}


/// # Implement `Power` for numbers.
pub trait Power {	
	/// Define the return type of the method `power_real`.
	type RealResult;
	/// Define the return type of the method `power`.
	type Result;

	/// Raise `self` by the power of a `Real` exponent `x`.
	/// ```maths, ignore
	/// self ^ x
	/// ```
	fn power_real(self: &Self, x: Real) -> Self::RealResult;

	/// Raise `self` by the power of an complex exponent `z`.
	/// ```maths, ignore
	/// self ^ z
	/// ```
	fn power<C, E>(self: &Self, z: C) -> Self::Result
	where 
		E: Complex,
		C: Complex + Exponentiation<Result = E>;
}


impl Exponentiation for Real {
	type Result = Real;

	#[inline]
	fn to_exp(self: &Self) -> Self::Result {
		self.exp()
	}
}

impl Power for Real {
	type RealResult = Real;
	type Result = Polar;

	#[inline]
	fn power_real(self: &Self, x: Real) -> Self::RealResult {
		self.powf(x)
	}

	/// See https://www.math.toronto.edu/mathnet/questionCorner/complexexp.html
	#[inline]
	fn power<C, E>(self: &Self, z: C) -> Self::Result
	where C: Complex + ToComplex
	{
		let ln: Real = self.ln();
		let exponent: C = z.real_multiplication(ln);
		exponent
			.to_algebraic()
			.to_exp()
	}
	
}

impl Exponentiation for Algebraic {
	type Result = Polar;

	#[inline]
	fn to_exp(self: &Self) -> Self::Result {
		Polar::new(
			self.imaginary(), 
			self.real().exp(),
		)
	}
}

impl Power for Algebraic {
	type RealResult = Polar;
	type Result = Polar;

	#[inline]
	fn power_real(self: &Self, x: Real) -> Self::RealResult {
		self
			.to_polar()
			.power_real(x)
	}

	fn power<C, E>(self: &Self, z: C) -> Self::Result
	where 
		E: Complex,
		C: Complex + Exponentiation<Result = E>,
	{
		self
			.to_polar()
			.power(z)	
	}
}

impl Exponentiation for Polar {
	type Result = Polar;
	
	#[inline]
	fn to_exp(self: &Self) -> Self::Result {
		self
			.to_algebraic()
			.to_exp()
	}
}

impl Power for Polar {
	type RealResult = Polar;
	type Result = Polar;
	
	#[inline]
	/// Raise `self` by the power of a `Real` exponent `x`.
	/// ```maths, ignore
	/// self ^ x
	/// r^x * e^(x * θ * i)
	/// ```
	fn power_real(self: &Self, x: Real) -> Self::RealResult {
		Polar { 
			theta: self.argument() * x, 
			distance: self.absolute().powf(x),
		}
	}

	/// Raise `self` by the power of a complex exponent `z`.
	/// ```maths, ignore
	///   self ^ z
	/// = (ze^(θi)) ^ z
	/// = r^z * e^(z * θ * i)
	/// = z1 * z2
	/// ```
	fn power<C, E>(self: &Self, z: C) -> Self::Result
	where 
		E: Complex + ToComplex,
		C: Complex + Exponentiation<Result = E>, 
	{
		let z1: E = z
			.real_multiplication(self.argument())
			.to_exp();
		let z2: Polar = self
			.absolute()
			.power(z);

		z1.to_polar() * z2
	}
}