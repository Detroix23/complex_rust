//! # Complex numbers.
//! src/operations/exponential.rs
//! 
//! Traits for `Exponentiation`, `Power`.

use crate::{Algebraic, Complex, Polar, Real, ToComplex, PI};

/// # Implement `Exponentiation` for numbers.
/// Play with Euler's number `e`.
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

/// # Implement `Logarithm`s.
pub trait Logarithm {
	type Result;

	/// Take the natural logarithm of `self`.
	/// With `Complex`es, defines the _ln_ of negative numbers and `Complex`es. 
	/// 
	/// **Panics** when:
	/// - `self` == `0`.
	/// 
	/// Sources.
	/// [Wikipedia](https://en.wikipedia.org/wiki/Complex_logarithm).
	fn ln(self: &Self) -> Self::Result;

	/// Take the logarithm with `base`. `base` can be complex.
	/// 
	/// **Mathematics**:
	/// Uses the natural logarithm:
	/// ```maths, ignore
	/// log_{b}(z1) = ln(z1) / ln(b)
	/// ```
	/// 
	/// **Generics**:
	/// - `C`: `base` type, can be any number, even `Complex`.
	/// - `L`: The result of a logarithm of `C`.
	fn log<C, L>(self: &Self, base: C) -> Self::Result
	where 
		L: Complex,
		C: Logarithm<Result = L>;
	
}

/// # Implement `Power` for numbers.
/// General exponentiation between complexes and reals.
/// ```maths, ignore
/// a ^ b
/// ```
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
	/// 
	/// A `Complex` power a `Complex` yields in theory an infinity amount of solution.
	///  
	/// Requires 2 generics, surely inferred:
	/// - `C`: `Complex` exponent;
	/// - `E`: `Complex` number result of an `Exponentiation` of `C`.
	fn power<C, E>(self: &Self, z: C) -> Self::Result
	where 
		E: Complex,
		C: Complex + Exponentiation<Result = E>;
}

// ===========
// Reals
// ===========

impl Exponentiation for Real {
	type Result = Real;

	#[inline]
	fn to_exp(self: &Self) -> Self::Result {
		self.exp()
	}
}

impl Logarithm for Real	{
	type Result = Algebraic;

	fn ln(self: &Self) -> Self::Result {
		if *self > 0.0 {
			Algebraic::new(Real::ln(*self), 0.0)
		} else if *self < 0.0 {
			Algebraic::new(Real::ln(*self), PI)
		} else {
			panic!("(X) ln(z): z == 0")
		}
	}

	fn log<C, L>(self: &Self, base: C) -> Self::Result
	where
		L: Complex, 
		C: Logarithm<Result = L>,
	{
		Logarithm::ln(self) / Logarithm::ln(&base)	
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
		let ln: Real = Real::ln(*self);
		let exponent: C = z.factor(ln);
		exponent
			.to_algebraic()
			.to_exp()
	}
	
}

// ===========
// Algebraic
// ===========

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

impl Logarithm for Algebraic {
	type Result = Algebraic;

	fn ln(self: &Self) -> Self::Result {
		self.to_polar().ln()
	}

	fn log<C, L>(self: &Self, base: C) -> Self::Result
	where 
		L: Complex,
		C: Logarithm<Result = L> 
	{
		Logarithm::ln(self) / Logarithm::ln(&base)		
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

// ===========
// Polar
// ===========

impl Exponentiation for Polar {
	type Result = Polar;
	
	#[inline]
	fn to_exp(self: &Self) -> Self::Result {
		self
			.to_algebraic()
			.to_exp()
	}
}

impl Logarithm for Polar {
	type Result = Algebraic;

	fn ln(self: &Self) -> Self::Result {
		if self.distance > 0.0 {
			Logarithm::ln(&self.distance) + Algebraic::new(0.0, self.argument())
		} else {
			Logarithm::ln(&self.distance.abs()) + Algebraic::new(
				0.0, 
				self.argument() - PI
			)
		}
	}

	fn log<C, L>(self: &Self, base: C) -> Self::Result
	where 
		L: Complex,
		C: Logarithm<Result = L> 
	{
		Logarithm::ln(self) / Logarithm::ln(&base)		
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
	/// 
	/// It exists an infinity of results. The function returns:
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
		let z1: Polar = self
			.absolute()
			.power(z);

		let z2: Polar = (z.to_algebraic() * Algebraic::new(0.0, 1.0))
			.factor(self.argument())
			.to_exp();
		
		z1 * z2
	}
}