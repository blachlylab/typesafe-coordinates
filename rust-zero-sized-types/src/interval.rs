use crate::coordinate::*;
use crate::basis::*;
use crate::end::*;

pub struct Interval<Bs: Basis, Ed: End>{
    ekind : Ed,
    start : Coordinate<Bs>,
    end : Coordinate<Bs>
}

pub type ZBHO = Interval<ZeroBased, HalfOpen>;
pub type OBHO = Interval<OneBased, HalfOpen>;
pub type ZBC = Interval<ZeroBased, Closed>;
pub type OBC = Interval<OneBased, Closed>;

impl<Bs: Basis, Ed: End> Interval<Bs,Ed>{
    pub fn new() -> Self {
        Interval{
            ekind: Ed::new(),
            start: Coordinate::<Bs>::new(),
            end: Coordinate::<Bs>::new(),
        }
    }

    pub fn from_int(start: i64, end: i64) -> Self {
        Interval{
            ekind: Ed::new(),
            start: Coordinate::<Bs>::from_int(start),
            end: Coordinate::<Bs>::from_int(end),
        }
    }
}

impl Interval<ZeroBased, HalfOpen>{
    pub fn to<ToBs: Basis, ToEd: End>(&self) -> Interval<ToBs, ToEd>{
        let mut ret = Interval::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
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

impl Interval<OneBased, HalfOpen>{
    fn to<ToBs: Basis, ToEd: End>(&self) -> Interval<ToBs, ToEd>{
        let mut ret = Interval::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
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

impl Interval<ZeroBased, Closed>{
    fn to<ToBs: Basis, ToEd: End>(&self) -> Interval<ToBs, ToEd>{
        let mut ret = Interval::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
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

impl Interval<OneBased, Closed>{
    fn to<ToBs: Basis, ToEd: End>(&self) -> Interval<ToBs, ToEd>{
        let mut ret = Interval::<ToBs, ToEd>::from_int(self.start.pos, self.end.pos);
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


impl<Bs: Basis, Ed: End> PartialEq for Interval<Bs, Ed>{
    fn eq(&self, other: &Interval<Bs, Ed>) -> bool{
        if self.start == other.start && self.end == other.end { true }
        else { false }
    }
}
