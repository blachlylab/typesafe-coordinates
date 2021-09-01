# Typesafe Coordinate Systems in High-Throughput Sequencing Applications

## Abstract
High-throughput sequencing file formats and tools encode coordinate intervals with respect to a reference sequence in at least four distinct, incompatible ways.
Integrating data from and moving data between different formats has the potential to introduce subtle off-by-one errors.
Here, we introduce the notion of typesafe coordinates: coordinate intervals are not only an integer pair, but members of a type class comprising four types: the Cartesian product of a zero or one basis, and an open or closed interval end.
By leveraging the type system of statically and strongly-typed, compiled languages we can provide static guarantees that an entire class of error is eliminated.
We provide a reference implementation in D as part of a larger work (dhtslib), and a proofs of concept in Rust, Python, and OCaml.
Exploratory implementations are available at https://github.com/blachlylab/typesafe-coordinates.

Preprint manuscript: (in process)

## Implementation notes

### D

The reference implementation of Typesafe Coordinate Systems is the `coordinates` module of `dhtslib` (https://github.com/blachlylab/dhtslib/),
a fully-featured high-throughput sequencing package.
Here, the `Coordinate` and `Interval` types are tightly integrated into the high-level API of `dhtslib` in order to guarantee correctness at compile time,
make implicit assumptions explicit in code, and deterministically propagate correctness (or lack thereof) thorughout the call chain and program flow to simplify debugging.
Briefly, Intervals are templated on Enum value describing the coordinate system; generic functions then leverage compile time metaprogramming for differential
coordinate arithmetic according to type.

### Rust

Proof-of-concept implementation #1: Four zero-sized types (empty structs), corresponding to the four coordinate systems parameterize an `Interval` type.
Runtime introspection and runtime branching is required inside generic functions, which is a disadvantage compared to D.
It is possible this could be written with type specialized generic functions, although it would be verbose.

Proof-of-concept implementation #2: Using the unstable nightly feature "const generics," the `Interval` type is parameterized on two Enums: `Basis` and `Openness`.
Again, runtime introspection of enum value and runtime branching is required inside generic functions, a disadvantage compared to the D version.
It is possible this could be written with type specialized generic functions, although it would be verbose.

### OCaml

Similar to the Rust POC #1, Ocaml `Coordinate` and `Interval` types are parameterized on zero-sized types.
Because OCaml does not have polymorphic function overloading (i.e., `foo` can not be defined to take both type `A` and type `B`)
there is some increased verbosity of free function names, like `overlaps_open`. It is possible this could be fixed using
modules, but this would require someone much more experienced in OCaml to implement.
Otherwise, due to type inference, the OCaml code that uses the types appears terse and elegant:
```OCaml
let i = make_zbho 0 100
let j = ob_interval_of_zb i;;
```

### Python

Clearly this is not "type safe," as python function parameters are not typed. In the provided implementation, a single class is used,
with member variables tracking basis, openness, and coordinates. This could be improved via separate (and tedious) implementation of four SEPARATE classes,
augmentation of functions with  optional type annotations in Python 3, and then the application of mypy (http://mypy-lang.org/) for type checking.
However, given the duck-typed nature of python, this appears to be a heavy lift for benefit, compared to using a true statically typed language.
