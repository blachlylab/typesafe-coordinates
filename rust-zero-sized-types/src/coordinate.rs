use crate::basis::*;

pub struct Coordinate<Bs: Basis>{
    kind : Bs,
    pub pos : i64
}

pub type ZB = Coordinate<ZeroBased>;
pub type OB = Coordinate<OneBased>;

impl<Bs: Basis> Coordinate<Bs>{
    pub fn new() -> Self {
        Coordinate{
            kind: Bs::new(),
            pos: -1
        }
    }

    pub fn from_int(pos: i64) -> Self {
        Coordinate{
            kind: Bs::new(),
            pos: pos
        }
    }
}

impl Coordinate<ZeroBased>{
    pub fn to<ToBs: Basis>(&self) -> Coordinate<ToBs>{
        let mut ret = Coordinate::<ToBs>::from_int(self.pos);
        if !ToBs::IS_ZERO_BASED {
            ret.pos += 1;
        }
        ret
    }
}

impl Coordinate<OneBased>{
    pub fn to<ToBs: Basis>(&self) -> Coordinate<ToBs>{
        let mut ret = Coordinate::<ToBs>::from_int(self.pos);
        if ToBs::IS_ZERO_BASED {
            ret.pos -= 1;
        }
        ret
    }
}

impl<T: Basis> PartialEq for Coordinate<T>{
    fn eq(&self, other: &Coordinate<T>) -> bool{
        if self.pos == other.pos { true }
        else { false }
    }
}
