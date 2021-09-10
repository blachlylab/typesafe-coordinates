Typesafe Coordinates implemented using the Rust (nightly unstable) feature "Const generics"

Parameterizing generics with a const value is akin to value templates in C++ and D,
which is how we implemented Typesafe Coordinates in `dhtslib`.

Implemented:
 - `Coordinate`
 - `Interval`
 - `Interval` constructors for each of the four coordinate systems
 - Comparison operators
 - `Interval::len`

TODO:
 - [x] Type conversion functions
 - [x] Interval::overlaps
 - [ ] Additional interval functions (intersect, union)
 - [ ] macros to wrap Rust-htslib functions?

Notes:
 - If matching on the const generic value, there is a runtime value check on the enum parameter and associated branching (confirmed via disassembly at godbolt.org) -- in D this could be checked at compile time.
 - Requires Rust nightly toolchain (`rustup default nightly` or `cargo +nightly test --lib`
 - (Sept 6, 2021) Rust has recently removed the feature flag and split `const_generics` into two
   * `adt_const_params` <- this is needed for our impl wherein we parameterize generics on Enum values
   * `generic_const_exprs`



References:
 - https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html
