# Complex numbers.

Started in 2026, _Rust edition 2024_.

Repository: 
- [Github](https://github.com/Detroix23/complex_rust);
- [crates.io] (WIP).

## Introduction.
Implements complex numbers computation in Rust.  
Compatibility with basic operations, using `float`s defined by type `Real`, such as:
- Comparison,
- Arithmetics,
- Module (absolute value),
- Argument.

## Structures.
Number themselves:
- `Algebraic`:
	- Simplest complex number: $a + ib$ ;
	- _Rectangular_ or _Cartesian_ form with a `real` part $a$ and `imaginary` part $b$.
- `Polar`:
	- Written: $r×e^{θi}$;
	- Made of a `distance` or `absolute` ($r$) and a angle `theta` or `argument` ($θ$).

Traits and type:
- `Real`: shared **type** to define a real number (usually a `float`);
- `Number`: **trait** to encapsulate common behavior of any number, real or complex (basic operators, comparison);
- `Complex`: **trait** to define any complex number (get real, imaginary, argument, absolute values);
- `ToComplex`: **trait** to parse from and between types.

Fundamental operations traits:
- `Exponentiation`: **trait**;
- `Logarithm`: **trait**;
- `Power`: **trait**;
- `Trigonometric`: **trait**;
- `Hyperbolic`: **trait**;
