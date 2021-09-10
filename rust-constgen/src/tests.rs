#[cfg(test)]
use super::*;

#[test]
fn test_display() {
    let x = ZBHO(100, 200);
    let y = ZBC(100, 199);
    let z = OBHO(100, 200);

    assert_eq!(format!("{}", x), "[100, 200)");
    assert_eq!(format!("{}", y), "[100, 199]");
    assert_eq!(format!("{}", x), format!("{}", z));
}

#[test]
fn coord_compare() {
    let x: Coordinate<{ Basis::Zero }> = Coordinate { pos: 0 };
    let y: Coordinate<{ Basis::Zero }> = Coordinate { pos: 100 };
    let z: Coordinate<{ Basis::Zero }> = Coordinate { pos: 200 };

    assert!(x < y);
    assert!(x <= z);
    assert!(y > x);
    assert!(z >= z);
    assert!(z == z);
    assert!(z >= y);

    let a: Coordinate<{ Basis::One }> = Coordinate { pos: 1 };
    let b: Coordinate<{ Basis::One }> = Coordinate { pos: 100 };

    assert!(a < b);
    assert!(b > a);
    assert!(a != b);

    // Won't compile:
    // assert!(y == b);    // different Basis => Different Type
}

#[test]
fn make_coords() {
    // Specify basis for ::new, infer for ::from_int
    let mut a = Coordinate::<{Basis::Zero}>::new();
    a.pos = 123;

    let b = Coordinate::from_int(123);
    assert_eq!(b.pos, 123);
    assert_eq!(a, b);

    // Infer basis for ::new, specify for ::from_int
    let mut c = Coordinate::new();
    c.pos = 456;

    let d = Coordinate::<{Basis::One}>::from_int(456);
    assert_eq!(d.pos, 456);
    assert_eq!(c, d);
}

#[test]
fn interval_compare() {
    let i: Interval<{ Basis::Zero }, { Openness::HalfOpen }> = Interval { start: 0, end: 100 };
    let j: Interval<{ Basis::Zero }, { Openness::HalfOpen }> = Interval {
        start: 50,
        end: 150,
    };
    let k: Interval<{ Basis::Zero }, { Openness::HalfOpen }> = Interval::from_int(100, 200);

    assert_eq!(i, i);
    assert_ne!(i, j);
    assert!(i < j);
    assert!(j < k);
    assert!(k >= k);
    assert!(k > i);

    let l: Interval<{ Basis::Zero }, { Openness::Closed }> = Interval {
        start: 100,
        end: 200,
    };
    let m: Interval<{ Basis::Zero }, { Openness::Closed }> = Interval::from_int(100, 200);
    assert_eq!(l, m);

    // Won't compile:
    // assert!(k != l);    // different Openness => Different Type

    // Try to trigger compiler bug (warning: associated function is never used: `new`)
    // by proving that it is called for all four combinations of (Basis, Openness)
    let _w: Interval<{ Basis::Zero }, { Openness::HalfOpen }> = Interval::new();
    let _x: Interval<{ Basis::Zero }, { Openness::Closed }> = Interval::new();
    let _y: Interval<{ Basis::One }, { Openness::HalfOpen }> = Interval::new();
    let _z: Interval<{ Basis::One }, { Openness::Closed }> = Interval::new();
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

#[test]
fn test_overlaps() {
    let a = ZBHO(0, 100);
    let b = ZBHO(50, 150);
    let c = ZBHO(100, 200);

    assert!(a.overlaps(&b));
    assert!(b.overlaps(&c));
    assert!(!a.overlaps(&c)); // in half open space [0,100) doesn't overlaps [100, 200)

    // check reverse
    assert!(b.overlaps(&a));

    let d = OBC(1, 100);
    let e = OBC(51, 150);
    let f = OBC(100, 200);

    assert!(d.overlaps(&e));
    assert!(e.overlaps(&f));
    assert!(d.overlaps(&f)); // in closed space, [1,100] overlaps [100,200]

    // check reverse
    assert!(e.overlaps(&d));
}

#[test]
fn test_conversion() {
    let x = ZBHO(0, 100);
    let y = x.to_closed();
    assert_eq!(y, ZBC(0, 99));

    let z = y.to_halfopen();
    assert_eq!(z, x);

    let a = OBC(1, 100);
    let b = x.to_onebasis().to_closed();
    assert_eq!(a, b);

    let c = a.to_zerobasis().to_halfopen();
    assert_eq!(c, x);
}
