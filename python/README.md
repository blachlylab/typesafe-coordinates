# Python Typed Coordinates

Due to nature of the Python language, this implementation is not explicitly type-safe nor executed at compile-time (as python is an interpreted language). Though it should still be robust enough to help solve most off-by-one errors.

Alternatively, an implementation could use something like `pyd` or `cython` to adapt another type-safe coordinates implementation. Adapting another implementation from a language with static typing would achieve the desired type-safety while keeping python's flexibility and ease of use.
