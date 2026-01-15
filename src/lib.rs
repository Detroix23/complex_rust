//! # Complex numbers.
//! src/lib.rs
//! 
//! A simple create to use and compute complex numbers.

mod base;

pub use crate::base::{
    common::Shared,
    types::Real,
    algebraic::Algebraic,

};


#[cfg(test)]
mod tests {
    use super::*;

    /// Test the bases of `Algebraic`.
    #[test]
    fn test_algebraic1() {

        let z1 = Algebraic::new(2.0, 1.0);
        let z4 = Algebraic::new(3.0, 4.0);
        let z2 = Algebraic::new(Real::sqrt(2.0) / -2.0, Real::sqrt(2.0) / -2.0);
        let z3 = Algebraic::new(Real::sqrt(2.0) / 2.0, Real::sqrt(2.0) / 2.0);
        

        println!("test_algebraic1 - z1 = {}, |z1| = {}", z1, z1.absolute());
        println!("test_algebraic1 - z2 = {}, |z2| = {}", z2, z2.absolute());
        println!("test_algebraic1 - z3 = {}, |z3| = {}", z3, z3.absolute());
        println!("test_algebraic1 - z4 = {}, |z4| = {}", z4, z4.absolute());

        println!("test_algebraic1 - z1 = {:?}, z2 = {:?}", z1, z2);
        println!("test_algebraic1 - z1 == z2 ? {}", z1 == z2);
        println!("test_algebraic1 - z3 == z2 ? {}", z3 == z2);

        println!("test_algebraic1 - (-z1) = {}", -z1);
        println!("test_algebraic1 - z1 + z4 = {}", z1 + z4);
        println!("test_algebraic1 - z1 - z4 = {}", z1 - z4);
        println!("test_algebraic1 - z1 * z4 = {}", z1 * z4);
        println!("test_algebraic1 - z1 / z4 = {}", z1 / z4);

        println!("test_algebraic1 - z4.conjugate() = {}, |z4.conjugate()| = {}", z4.conjugate(), z4.conjugate().absolute());
        println!("test_algebraic1 - z1 / z4.conjugate() = {}", z1 / z4.conjugate());
        println!("test_algebraic1 - z2 * z3 = {}", z2 * z3);
        println!("test_algebraic1 - z1 / z4 + z2 * z3 = {}", z1 / z4 + z2 * z3);

        println!("test_algebraic1 - z1.argument() = {}", z1.argument());
        println!("test_algebraic1 - z2.argument() = {}", z2.argument());
        println!("test_algebraic1 - z3.argument() = {}", z3.argument());
        println!("test_algebraic1 - z4.argument() = {}", z4.argument());
        println!("test_algebraic1 - (0 + 1i).argument() = {}", Algebraic::new(0.0, 1.0).argument());
        println!("test_algebraic1 - (-2 + 0i).argument() = {}", Algebraic::new(-2.0, 0.0).argument());
        println!("test_algebraic1 - (-2 - 2i).argument() = {}", Algebraic::new(-2.0, -2.0).argument());
    }
}
