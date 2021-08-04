use crate::coordinate::*;
use crate::basis::*;
use crate::end::*;

pub struct Coordinates<Bs: Basis, Ed: End>{
    bkind : Bs,
    ekind : Ed,
    start : Coordinate<Bs>,
    end : Coordinate<Bs>
}

type ZBHO = Coordinates<ZeroBased, HalfOpen>;
type OBHO = Coordinates<OneBased, HalfOpen>;
type ZBC = Coordinates<ZeroBased, Closed>;
type OBC = Coordinates<OneBased, Closed>;

impl<Bs: Basis, Ed: End> Coordinates<Bs,Ed>{
    pub fn new() -> Self {
        Coordinates{
            bkind: Bs::one(),
            ekind: Ed::one(),
            start: Coordinate::<Bs>::new(),
            end: Coordinate::<Bs>::new(),
        }
    }

    pub fn from_int(start: i64, end: i64) -> Self {
        Coordinates{
            bkind: Bs::one(),
            ekind: Ed::one(),
            start: Coordinate::<Bs>::from_int(start),
            end: Coordinate::<Bs>::from_int(end),
        }
    }
}

impl Coordinates<ZeroBased, HalfOpen>{
    pub fn to<ToBs: Basis, ToEd: End>(&self) -> Coordinates<ToBs, ToEd>{
        let mut ret = Coordinates::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
        if !ToBs::IS_ZERO_BASED {
            ret.start.pos += 1;
            ret.end.pos += 1;
        }
        if !ToEd::IS_EXCLUSIVE {
            ret.end.pos -= 1;
        }
        ret 
    }
}

impl Coordinates<OneBased, HalfOpen>{
    fn to<ToBs: Basis, ToEd: End>(&self) -> Coordinates<ToBs, ToEd>{
        let mut ret = Coordinates::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
        if ToBs::IS_ZERO_BASED {
            ret.start.pos -= 1;
            ret.end.pos -= 1;
        }
        if !ToEd::IS_EXCLUSIVE {
            ret.end.pos -= 1;
        }
        ret 
    }
}

impl Coordinates<ZeroBased, Closed>{
    fn to<ToBs: Basis, ToEd: End>(&self) -> Coordinates<ToBs, ToEd>{
        let mut ret = Coordinates::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
        if !ToBs::IS_ZERO_BASED {
            ret.start.pos += 1;
            ret.end.pos += 1;
        }
        if ToEd::IS_EXCLUSIVE {
            ret.end.pos += 1;
        }
        ret 
    }
}

impl Coordinates<OneBased, Closed>{
    fn to<ToBs: Basis, ToEd: End>(&self) -> Coordinates<ToBs, ToEd>{
        let mut ret = Coordinates::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
        if !ToBs::IS_ZERO_BASED {
            ret.start.pos -= 1;
            ret.end.pos -= 1;
        }
        if ToEd::IS_EXCLUSIVE {
            ret.end.pos += 1;
        }
        ret 
    }
}


impl<Bs: Basis, Ed: End> PartialEq for Coordinates<Bs, Ed>{
    fn eq(&self, other: &Coordinates<Bs, Ed>) -> bool{
        if self.start == other.start && self.end == other.end { true }
        else { false }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn coordinates_tests1() {
        let c0 = Coordinates::<ZeroBased, HalfOpen>::from_int(1, 3);
        let c1 = Coordinates::<ZeroBased, Closed>::from_int(1, 2);
        let c2 = Coordinates::<OneBased, HalfOpen>::from_int(2, 4);
        let c3 = Coordinates::<OneBased, Closed>::from_int(2, 3);
        assert!(c0.to::<ZeroBased, HalfOpen>() == c0);
        assert!(c0.to::<ZeroBased, Closed>() == c1);
        assert!(c0.to::<OneBased, HalfOpen>() == c2);
        assert!(c0.to::<OneBased, Closed>() == c3);
        
    }

    #[test]
    fn coordinates_tests2() {
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