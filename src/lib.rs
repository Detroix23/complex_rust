//! # Complex numbers.
//! src/lib.rs
//! 
//! A simple create to use and compute complex numbers.

mod base;

pub use crate::base::defaults::Real;
pub use crate::base::common::Complex;
pub use crate::base::algebraic::Algebraic;
pub use crate::base::polar::Polar;


#[cfg(test)]
mod tests {
    use super::*;

    /// Test the bases of `Algebraic`.
    #[test]
    fn test_algebraic1() {
        println!("# Test: Algebraic 1.");

        let z1 = Algebraic::new(2.0, 1.0);
        let z4 = Algebraic::new(3.0, 4.0);
        let z2 = Algebraic::new(Real::sqrt(2.0) / -2.0, Real::sqrt(2.0) / -2.0);
        let z3 = Algebraic::new(Real::sqrt(2.0) / 2.0, Real::sqrt(2.0) / 2.0);
        

        println!("z1 = {}, |z1| = {}", z1, z1.absolute());
        println!("z2 = {}, |z2| = {}", z2, z2.absolute());
        println!("z3 = {}, |z3| = {}", z3, z3.absolute());
        println!("z4 = {}, |z4| = {}", z4, z4.absolute());

        println!("z1 = {:?}, z2 = {:?}", z1, z2);
        println!("z1 == z2 ? {}", z1 == z2);
        println!("z3 == z2 ? {}", z3 == z2);

        println!("(-z1) = {}", -z1);
        println!("z1 + z4 = {}", z1 + z4);
        println!("z1 - z4 = {}", z1 - z4);
        println!("z1 * z4 = {}", z1 * z4);
        println!("z1 / z4 = {}", z1 / z4);

        println!("z4.conjugate() = {}, |z4.conjugate()| = {}", z4.conjugate(), z4.conjugate().absolute());
        println!("z1 / z4.conjugate() = {}", z1 / z4.conjugate());
        println!("z2 * z3 = {}", z2 * z3);
        println!("z1 / z4 + z2 * z3 = {}", z1 / z4 + z2 * z3);

        println!("z1.argument() = {}", z1.argument());
        println!("z2.argument() = {}", z2.argument());
        println!("z3.argument() = {}", z3.argument());
        println!("z4.argument() = {}", z4.argument());
        println!("(0 + 1i).argument() = {}", Algebraic::new(0.0, 1.0).argument());
        println!("(-2 + 0i).argument() = {}", Algebraic::new(-2.0, 0.0).argument());
        println!("(-2 - 2i).argument() = {}", Algebraic::new(-2.0, -2.0).argument());

        println!("z1.distance_to(z4) = {}", z1.distance_to(z4))
    }
}
