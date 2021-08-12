use crate::coordinate::*;
use crate::basis::*;
use crate::end::*;

pub struct Coordinates<Bs: Basis, Ed: End>{
    bkind : Bs,
    ekind : Ed,
    start : Coordinate<Bs>,
    end : Coordinate<Bs>
}

pub type ZBHO = Coordinates<ZeroBased, HalfOpen>;
pub type OBHO = Coordinates<OneBased, HalfOpen>;
pub type ZBC = Coordinates<ZeroBased, Closed>;
pub type OBC = Coordinates<OneBased, Closed>;

impl<Bs: Basis, Ed: End> Coordinates<Bs,Ed>{
    pub fn new() -> Self {
        Coordinates{
            bkind: Bs::new(),
            ekind: Ed::new(),
            start: Coordinate::<Bs>::new(),
            end: Coordinate::<Bs>::new(),
        }
    }

    pub fn from_int(start: i64, end: i64) -> Self {
        Coordinates{
            bkind: Bs::new(),
            ekind: Ed::new(),
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
