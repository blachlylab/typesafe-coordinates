#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![allow(non_snake_case)] // ZBHO, OBC, etc. match dhtslib interface

/// Typesafe Coordinates using Rust (nightly) feature "const generics".
///
/// Const generics are akin to C++/D templating on value, and similar to
/// how we implemented Typesafe Coordinates in D.
///
/// Examples:
///
/// let i : Interval<{Basis::Zero}, {Openness::HalfOpen}> = Interval::from_int(0, 100);
///
/// # Conversion
/// To avoid writing a series of 12 distinct functions (Rust lacks compile time `static if`),
/// conversions are, as in the OCaml implementation, specialized on only one of Basis or Openness.
/// Then, we only need 4 conversion functions: `to_onebasis`, `to_zerobasis`, `to_closed`, `to_halfopen`.
/// Conversion can be done in one step or two, as needed. If we wished, we could write a series of
/// four functions `to_{completetype}`, and use runtime matching on the type's const enum parameter,
/// but again this incurs runtime branch cost as Rust does not have D's `static if` construct.
///
/// # Notes
/// The rust nightly compiler as of 1.56 erroneously emits `dead_code` warning for methods on
/// generic structs parameterized by enums? Alternatively is this on all methods when run
/// only via testing?
mod tests;

use std::cmp::Ordering;
use std::fmt; // `fmt::Display` trait

#[derive(Eq, PartialEq)]
pub enum Basis {
    Zero,
    One,
}

#[derive(Eq, PartialEq)]
pub enum Openness {
    HalfOpen,
    Closed,
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
    fn new() -> Self {
        Coordinate { pos: 0 }
    }

    fn from_int(pos: u64) -> Self {
        Coordinate { pos }
    }
}

/// A genomic interval
///
/// Here, we parameterized with two phantom types (values),
/// but another choice would ahve been to actually embed two
/// `Coordinate<B>` as the `start` and `end`.
/// Unclear whether that helps, hurts, or is neutral to this implementation.
///
/// Additional future improvements could be to embed an Option<ContigName>,
/// or even Genome Build information. Given that coordinates may be constructed
/// often and quickly in a hot loop if processing a large NGS file, that sort
/// of potential overhead should ideally be optional.
#[derive(Debug, Eq, PartialEq)]
pub struct Interval<const B: Basis, const O: Openness> {
    start: u64,
    end: u64,
}

/// Note that the selection of end marker (paren or bracket) according to Openness is at runtime
/// We could of course write this twice, but nah.
impl<const B: Basis, const O: Openness> fmt::Display for Interval<B, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let endchar = match O {
            Openness::HalfOpen => ")",
            Openness::Closed => "]",
        };
        write!(f, "[{}, {}{}", self.start, self.end, endchar)
    }
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

/// Compile-time zero-basis specializations
impl<const O: Openness> Interval<{ Basis::Zero }, O> {
    fn to_onebasis(&self) -> Interval<{ Basis::One }, O> {
        Interval {
            start: self.start + 1,
            end: self.end + 1,
        }
    }
}

/// Compile-time one-basis speicalizations
impl<const O: Openness> Interval<{ Basis::One }, O> {
    fn to_zerobasis(&self) -> Interval<{ Basis::Zero }, O> {
        Interval {
            start: self.start - 1,
            end: self.end - 1,
        }
    }
}

/// Compile time half-open specializations
///
/// Overlap detection differs depending on openness of range end
impl<const B: Basis> Interval<B, { Openness::HalfOpen }> {
    fn len(&self) -> u64 {
        self.end - self.start
    }

    fn overlaps(&self, other: &Self) -> bool {
        if self.start > other.start {
            let min = other;
            let max = self;
            min.end > max.start
        } else {
            let min = self;
            let max = other;
            min.end > max.start
        }
    }

    /// Convert to a closed interval of same start type
    fn to_closed(&self) -> Interval<B, { Openness::Closed }> {
        Interval {
            start: self.start,
            end: self.end - 1,
        }
    }
}

/// Compile time closed specializations
///
/// Overlap detection differs depending on openness of range end
impl<const B: Basis> Interval<B, { Openness::Closed }> {
    fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    fn overlaps(&self, other: &Self) -> bool {
        if self.start > other.start {
            let min = other;
            let max = self;
            min.end >= max.start
        } else {
            let min = self;
            let max = other;
            min.end >= max.start
        }
    }

    /// Convert to a half-open interval of same start type
    fn to_halfopen(&self) -> Interval<B, { Openness::HalfOpen }> {
        Interval {
            start: self.start,
            end: self.end + 1,
        }
    }
}

impl<const B: Basis, const O: Openness> Interval<B, O> {
    fn new() -> Self {
        Interval { start: 0, end: 0 }
    }

    fn from_int(start: u64, end: u64) -> Self {
        Interval { start, end }
    }
}

/// Constuct a Zero-based, Half Open Interval
pub fn ZBHO(start: u64, end: u64) -> Interval<{ Basis::Zero }, { Openness::HalfOpen }> {
    debug_assert!(end > start || start == 0);
    Interval { start, end }
}

/// Construct a Zero-based, Closed Interval
pub fn ZBC(start: u64, end: u64) -> Interval<{ Basis::Zero }, { Openness::Closed }> {
    debug_assert!(end > start || start == 0);
    Interval { start, end }
}

/// Construct a One-based, Half Open Interval
pub fn OBHO(start: u64, end: u64) -> Interval<{ Basis::One }, { Openness::HalfOpen }> {
    debug_assert!(start > 0);
    debug_assert!(end >= start);
    Interval { start, end }
}

/// Construct a One-based, Closed Interval
pub fn OBC(start: u64, end: u64) -> Interval<{ Basis::One }, { Openness::Closed }> {
    debug_assert!(start > 0);
    debug_assert!(end >= start);
    Interval { start, end }
}
