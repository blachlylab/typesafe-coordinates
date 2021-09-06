#[cfg(test)]

use super::*;

#[test]
fn coord_compare() {
    let x : Coordinate<{Basis::Zero}> = Coordinate{ pos: 0 };
    let y : Coordinate<{Basis::Zero}> = Coordinate{ pos: 100 };
    let z : Coordinate<{Basis::Zero}> = Coordinate{ pos: 200 };

    assert!(x < y);
    assert!(x <= z);
    assert!(y > x);
    assert!(z >= z);
    assert!(z == z);
    assert!(z >= y);

    let a: Coordinate<{Basis::One}> = Coordinate{ pos: 1 };
    let b: Coordinate<{Basis::One}> = Coordinate{ pos: 100 };

    assert!(a < b);
    assert!(b > a);
    assert!(a != b);

    // Won't compile:
    // assert!(y == b);    // different Basis => Different Type
}

#[test]
fn interval_compare() {
    let i : Interval<{Basis::Zero}, {Openness::HalfOpen}> = Interval{ start: 0, end: 100 };
    let j : Interval<{Basis::Zero}, {Openness::HalfOpen}> = Interval{ start: 50, end: 150 };
    let k : Interval<{Basis::Zero}, {Openness::HalfOpen}> = Interval::from_int(100, 200);
    
    assert_eq!(i, i);
    assert_ne!(i, j);
    assert!(i < j);
    assert!(j < k);
    assert!(k >= k);
    assert!(k > i);

    let l : Interval<{Basis::Zero}, {Openness::Closed}> = Interval{ start: 100, end: 200};
    let m : Interval<{Basis::Zero}, {Openness::Closed}> = Interval::from_int(100, 200);
    assert_eq!(l, m);

    // Won't compile:
    // assert!(k != l);    // different Openness => Different Type

    // Try to trigger compiler bug (warning: associated function is never used: `new`)
    // by proving that it is called for all four combinations of (Basis, Openness)
    let _w : Interval<{Basis::Zero}, {Openness::HalfOpen}> = Interval::new();
    let _x : Interval<{Basis::Zero}, {Openness::Closed}> = Interval::new();
    let _y : Interval<{Basis::One}, {Openness::HalfOpen}> = Interval::new();
    let _z : Interval<{Basis::One}, {Openness::Closed}> = Interval::new();
}

#[test]
fn test_constructors() {
    let _a = ZBHO(0, 100);
    let _b = ZBC(0, 100);
    let _c = OBHO(1, 100);
    let _d = OBC(1, 100);
}

#[test]
fn test_len() {
    let open = ZBHO(0, 100);
    let closed = ZBC(0, 100);

    assert_eq!(open.len(), 100);
    assert_eq!(closed.len(), 101);
    assert_ne!(open.len(), 101);
}
