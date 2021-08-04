
// https://rreverser.com/conditional-enum-variants-in-rust/
pub struct HalfOpen {}
pub struct Closed {}

pub trait End {
    const IS_EXCLUSIVE: bool;

    // https://stackoverflow.com/questions/53304428/why-do-i-get-the-error-expected-type-parameter-found-integral-variable
    fn new() -> Self;
}


impl End for HalfOpen
{
    const IS_EXCLUSIVE: bool = true;
    fn new() -> Self { HalfOpen {} }
}

impl End for Closed
{
    const IS_EXCLUSIVE: bool = false;
    fn new() -> Self { Closed {} }
}