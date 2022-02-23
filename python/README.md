# Python (not) Type-safe Coordinates

Due to limitations of the Python language, this implementation is not actually type-safe nor executed at compile-time (as python is an interpreted language). Though it should still be robust enough to help solve most off-by-one errors.

An option could be to use something like `pyd` or `cython` to adapt another type-safe coordinates implementation to use in python code.