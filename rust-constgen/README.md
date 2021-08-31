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
    - Type conversion functions

Notes:
    - Until const generics type specialization is implemented, there is a runtime value check on the enum parameter and associated branching (confirmed via disassembly at godbolt.org)

