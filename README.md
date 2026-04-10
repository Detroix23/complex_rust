# Complex numbers.

Started in 2026, _Rust edition 2024_.

Repository: 
- [Github](https://github.com/Detroix23/complex_rust);
- crates.io (WIP).

## Introduction.
Implements complex numbers computation in Rust.  
Compatibility with basic operations, using `float`s, such as:
- Comparison,
- Arithmetics,
- Module (absolute value),
- Argument.

## Structures.
Number themselves:
- `Algebraic`:
	- Simplest complex number: _a + ib_ ;
	- _Rectangular_ or _Cartesian_ form.
- `Polar`:
	- Made of a `distance` or `absolute` and a angle `theta` or `argument`.

Traits and type:
- `Real`: shared **type** to define a real number (usually a float);
- `Number`: **trait** to encapsulate common behavior of any number, real or complex;
- `Complex`: **trait** to define any complex number;
- `ToComplex`: **trait** to parse from and between types.
