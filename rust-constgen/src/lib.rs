#![allow(incomplete_features)]
#![feature(const_generics)]
#![allow(non_snake_case)]   // ZBHO, OBC, etc. match dhtslib interface

/// Typesafe Coordinates using Rust (nightly) feature "const generics".
///
/// Const generics are akin to C++/D templating on value, and similar to
/// how we implemented Typesafe Coordinates in D.
///
/// Examples:
/// 
/// let i : Interval<{Basis::Zero}, {Openness::HalfOpen}> = Interval::from_int(0, 100);

mod tests;

use std::cmp::Ordering;

// Compiler bug: warns `Variant is not constructed` for enum members,
// when used only at compile time to parameterize a type via const generics
#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Basis {
    Zero,
    One
}

// Compiler bug: warns `Variant is not constructed` for enum members,
// when used only at compile time to parameterize a type via const generics
#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Openness {
    HalfOpen,
    Closed
}

#[derive(Debug, Eq, PartialEq)]
pub struct Coordinate<const B: Basis> {
    pos: u64,
}

impl<const B: Basis> PartialOrd for Coordinate<B> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl<const B: Basis> Coordinate<B> {
    // Compiler warns not used I believe due to const generics.
    #[allow(dead_code)]
    fn new() -> Self {
        Coordinate {
            pos: 0
        }
    }

    // Compiler warns not used I believe due to const generics.
    #[allow(dead_code)]
    fn from_int(pos: u64) -> Self {
        Coordinate {
            pos
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Interval<const B: Basis, const O: Openness> {
    start: u64,
    end: u64
}

impl<const B: Basis, const O: Openness> PartialOrd for Interval<B, O> {
    /// Interval Ordering is based strictly on the start coordinate
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

// With runtime matching on generic parameterization
/*
impl<const B: Basis, const O: Openness> Interval<B, O> {
    fn len(&self) -> u64 {
        // Without const generic specialization (???), this is a runtime test
        match O {
            Openness::HalfOpen => self.end - self.start,
            Openness::Closed => self.end - self.start + 1,
        }
    }
}
*/

// Compile time
impl<const B: Basis> Interval<B, {Openness::HalfOpen}> {
    #[allow(dead_code)]
    fn len(&self) -> u64 {
        self.end - self.start
    }
}
// Compile time
impl<const B: Basis> Interval<B, {Openness::Closed}> {
    #[allow(dead_code)]
    fn len(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl<const B: Basis, const O: Openness> Interval<B, O> {
    // Compiler warns not used I believe due to const generic? Clearly is used in test.
    // Is also used in all 4 combinations, so is likely `rustc` bug
    #[allow(dead_code)]
    fn new() -> Self {
        Interval {
            start: 0,
            end: 0,
        }
    }

    // Compiler warns not used I believe due to const generics.
    #[allow(dead_code)]
    fn from_int(start: u64, end: u64) -> Self {
        Interval {
            start,
            end
        }
    }
}

/// Constuct a Zero-based, Half Open Interval
pub fn ZBHO(start: u64, end: u64) -> Interval<{Basis::Zero}, {Openness::HalfOpen}> {
    Interval { start, end }
}

/// Construct a Zero-based, Closed Interval
pub fn ZBC(start: u64, end: u64) -> Interval<{Basis::Zero}, {Openness::Closed}> {
    Interval { start, end }
}

/// Construct a One-based, Half Open Interval
pub fn OBHO(start: u64, end: u64) -> Interval<{Basis::One}, {Openness::HalfOpen}> {
    Interval { start, end }
}

/// Construct a One-based, Closed Interval
pub fn OBC(start: u64, end: u64) -> Interval<{Basis::One}, {Openness::Closed}> {
    Interval { start, end }
}

