//! # Complex numbers.
//! src/operations/mod.rs
//! 
//! Implement basic operations for complex numbers.

pub mod exponential;
pub mod trigonometry;

pub use exponential::{Exponentiation, Logarithm, Power};
pub use trigonometry::{Trigonometry, Hyperbolic};
