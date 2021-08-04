
// https://rreverser.com/conditional-enum-variants-in-rust/
pub struct ZeroBased {}
pub struct OneBased {}

pub trait Basis {
    const IS_ZERO_BASED: bool;

    // https://stackoverflow.com/questions/53304428/why-do-i-get-the-error-expected-type-parameter-found-integral-variable
    fn new() -> Self;
}

impl Basis for ZeroBased
{
    const IS_ZERO_BASED: bool = true;
    fn new() -> Self { ZeroBased {} }
}

impl Basis for OneBased
{
    const IS_ZERO_BASED: bool = false;
    fn new() -> Self { OneBased {} }
}