//! # Complex numbers.
//! src/lib.rs
//! 
//! A simple create to use and compute complex numbers.

pub mod base;
pub mod operations;

pub use base::defaults::{Real, PI};
pub use base::common::{Number, Complex, ToComplex};
pub use base::algebraic::Algebraic;
pub use base::polar::Polar;

pub use operations::exponential::{Exponentiation, Power, Logarithm};

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the bases of `Algebraic`.
    #[test]
    fn test_algebraic1() {

        let z0 = Algebraic::new(0.0, 0.0);
        let z1 = Algebraic::new(2.0, 1.0);
        let z2 = Algebraic::new(Real::sqrt(2.0) / -2.0, Real::sqrt(2.0) / -2.0);
        let z3 = Algebraic::new(Real::sqrt(2.0) / 2.0, Real::sqrt(2.0) / 2.0);
        let z4 = Algebraic::new(3.0, 4.0);
        let z5 = Algebraic::new(42.0, 0.0);
        let z6 = Algebraic::new(0.0, -32.0);
        let z7 = Algebraic::new(Real::sqrt(2.0) / -2.0, Real::sqrt(2.0) / -2.0);

        assert_eq!(z0.absolute(), 0.0);
        assert_eq!(z1.absolute(), 2.23606797749979);
        assert_eq!(z2.absolute(), 1.0);
        assert_eq!(z3.absolute(), 1.0); 
        assert_eq!(z4.absolute(), 5.0);
        assert_eq!(z5.absolute(), 42.0);
        assert_eq!(z6.absolute(), 32.0);
        
        assert_ne!(z0, z1);
        assert_ne!(z1, z2);
        assert_ne!(z5, z6);
        assert_eq!(z2, z7);

        assert_eq!(-z0, Algebraic::new(0.0, 0.0));
        assert_eq!(-z1, Algebraic::new(-2.0, -1.0));
        assert_eq!(-z5, Algebraic::new(-42.0, 0.0));
        assert_eq!(z1 + z4, Algebraic::new(5.0, 5.0));
        assert_eq!(z0 + z2, z2);
        assert_eq!(z1 - z4, Algebraic::new(-1.0, -3.0));
        assert_eq!(z1 * z4, Algebraic::new(2.0, 11.0));
        assert_eq!(z0 * z3, Algebraic::new(0.0, 0.0));
        assert_eq!(z1 / z4, Algebraic::new(0.4, -0.2));
        assert_eq!(z0 / z4, Algebraic::new(0.0, 0.0));
        assert_eq!(z2 * z3, Algebraic::new(0.0, -1.0));
        assert_eq!(z1 / z4 + z2 * z3, Algebraic::new(0.4, -1.2));

        assert_eq!(z4.conjugate(), Algebraic::new(3.0, -4.0));
        assert_eq!(z3, z3.conjugate().conjugate());
        assert_eq!(z0, z0.conjugate().conjugate());
        assert_eq!(z5, z5.conjugate().conjugate());
        assert_eq!(z6, z6.conjugate().conjugate());
        assert_eq!(z4.conjugate().absolute(), z4.absolute());

        assert_eq!(z1.argument(), 0.46364760900080615);
        assert_eq!(z2.argument(), -2.356194490192345);
        assert_eq!(z3.argument(), 0.7853981633974483);
        assert_eq!(z4.argument(), 0.9272952180016123);
        assert_eq!(Algebraic::new(0.0, 1.0).argument(), PI / 2.0);
        assert_eq!(Algebraic::new(-2.0, 0.0).argument(), PI);
        assert_eq!(Algebraic::new(-2.0, -2.0).argument(), (3.0 * -PI) / 4.0);
        assert!(z0.argument().is_nan());

        assert_eq!(z1.distance_to(z4), 3.1622776601683795);
    }

    #[test]
    fn test_exponents1() {

        let z1: Algebraic = Algebraic::new(2.0, 1.5);
        let z2: Polar = Polar::new(-1.0, 3.0);

        assert_eq!(z1.factor(-1.0), Algebraic::new(-2.0, -1.5));
        assert_eq!(z1.factor(1.0), Algebraic::new(2.0, 1.5));
        assert_eq!(z1.factor(0.0), Algebraic::new(0.0, 0.0));
        assert_eq!(z1.factor(2.0), Algebraic::new(4.0, 3.0));

        assert_eq!(2.0.power_real(4.0), 16.0);
        assert_eq!(2.0.power(z1).to_algebraic(), Algebraic::new(2.0258441836117975, 3.449051368670268));
        assert_eq!(3.1.power(z1).to_algebraic(), Algebraic::new(-1.2105839103907692, 9.533445683272289));
        let e1_4 = 2.0.power(z2).to_algebraic();
        assert_eq!(e1_4, Algebraic::new(-0.5475919974804363, -3.0265442727908844), "g: {}", e1_4);
        let e1_5 = 3.1.power(z2).to_algebraic();
        assert_eq!(e1_5, Algebraic::new(-6.004962520169716, -1.762349739543248), "g: {}", e1_5);
        
        assert_eq!(z1.power(z2).to_algebraic(), Algebraic::new(6.639957908875393, -21.40787487030071));
        assert_eq!(z2.power(z1).to_algebraic(), Algebraic::new(37.86091586986762, -13.909692314102696));
        assert_eq!(z1.power(z1).to_algebraic(), Algebraic::new(-2.111362375152732, 1.0996123663804858));
        assert_eq!(z2.power(z2).to_algebraic(), Algebraic::new(-0.1486914414860377, 0.4515154451580148));
    
    
        assert_eq!(z1.ln(),        Algebraic::new(0.9162907318741551, 0.6435011087932843));
        assert_eq!(z2.ln(),        Algebraic::new(1.0986122886681098, -1.0));
        assert_eq!(z1.log(2.0),    Algebraic::new(1.3219280948873624, 0.9283758584626206));
        assert_eq!(z1.log(10.0),   Algebraic::new(0.3979400086720376, 0.27946898064754744));
        assert_eq!(z2.log(2.0),    Algebraic::new(1.5849625007211563, -1.4426950408889634));
        assert_eq!(z2.log(10.0),   Algebraic::new(0.4771212547196624, -0.43429448190325176));
        assert_eq!(z1.log(z2),     Algebraic::new(0.1645471443538280, 0.7355172170217943));
        assert_eq!(z2.log(z2),     Algebraic::new(1.0, 0.0));
        assert_eq!(z1.log(z1),     Algebraic::new(1.0, 0.0));
    }

}
