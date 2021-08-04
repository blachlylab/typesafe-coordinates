// https://rreverser.com/conditional-enum-variants-in-rust/
pub struct ZeroBased {}
pub struct OneBased {}

pub trait Basis {
    const IS_ZERO_BASED: bool;
    fn one() -> Self;
}

impl Basis for ZeroBased
{
    const IS_ZERO_BASED: bool = true;
    fn one() -> Self {
        ZeroBased {}
    }
}

impl Basis for OneBased
{
    const IS_ZERO_BASED: bool = false;
    fn one() -> Self {
        OneBased {}
    }
}

pub struct Coordinate<Bs: Basis>{
    kind : Bs,
    pub pos : i64
}

type ZB = Coordinate<ZeroBased>;
type OB = Coordinate<OneBased>;

impl<Bs: Basis> Coordinate<Bs>{
    pub fn new() -> Self {
        Coordinate{
            kind: Bs::one(),
            pos: -1
        }
    }

    pub fn from_int(pos: i64) -> Self {
        Coordinate{
            kind: Bs::one(),
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn coordinate_tests1() {
        let c0 = Coordinate::<ZeroBased>::from_int(1);
        let c1 = Coordinate::<OneBased>::from_int(2);
        assert!(c0.to::<ZeroBased>() == c0);
        assert!(c0.to::<OneBased>() == c1);
        
    }

    #[test]
    fn coordinate_tests2() {
        let c0 = ZB::from_int(1);
        let c1 = OB::from_int(2);
        assert!(c0.to::<ZeroBased>() == c0);
        assert!(c0.to::<OneBased>() == c1);
    }
}