pub mod coordinate;
pub mod interval;
pub mod basis;
pub mod end;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::coordinate::*;
    use crate::interval::*;
    use crate::basis::*;
    use crate::end::*;

    #[test]
    fn test_coordinate_conversions() {
        let c0 = Coordinate::<ZeroBased>::from_int(1);
        let c1 = Coordinate::<OneBased>::from_int(2);
        assert!(c0.to::<ZeroBased>() == c0);
        assert!(c0.to::<OneBased>() == c1);
        
    }

    #[test]
    fn test_coordinate_shortform_conversions() {
        let c0 = ZB::from_int(1);
        let c1 = OB::from_int(2);
        assert!(c0.to::<ZeroBased>() == c0);
        assert!(c0.to::<OneBased>() == c1);
    }

    #[test]
    fn test_coordinate_pair_conversions() {
        let c0 = Interval::<ZeroBased, HalfOpen>::from_int(1, 3);
        let c1 = Interval::<ZeroBased, Closed>::from_int(1, 2);
        let c2 = Interval::<OneBased, HalfOpen>::from_int(2, 4);
        let c3 = Interval::<OneBased, Closed>::from_int(2, 3);
        assert!(c0.to::<ZeroBased, HalfOpen>() == c0);
        assert!(c0.to::<ZeroBased, Closed>() == c1);
        assert!(c0.to::<OneBased, HalfOpen>() == c2);
        assert!(c0.to::<OneBased, Closed>() == c3);
        
    }

    #[test]
    fn test_coordinate_pair_shortform_conversions() {
        let c0 = ZBHO::from_int(1, 3);
        let c1 = ZBC::from_int(1, 2);
        let c2 = OBHO::from_int(2, 4);
        let c3 = OBC::from_int(2, 3);
        assert!(c0.to::<ZeroBased, HalfOpen>() == c0);
        assert!(c0.to::<ZeroBased, Closed>() == c1);
        assert!(c0.to::<OneBased, HalfOpen>() == c2);
        assert!(c0.to::<OneBased, Closed>() == c3);
    }
}