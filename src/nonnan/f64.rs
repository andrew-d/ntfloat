use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use super::error::ParseNonNanError;


/// NonNan is a newtype that wraps an internal floating-point number, guaranteeing that it will
/// never be NaN (Not A Number).  As a result, it can provide a correct implementation of `Ord`
/// that is not well-defined on floating point numbers (since `NaN` does not compare).
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct NonNan(f64);

impl NonNan {
    /// Create a new `NonNan` instance with the given floating-point number.  Will return `None` if
    /// the input number is NaN.
    pub fn new(val: f64) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Deref for NonNan {
    type Target = f64;

    fn deref<'a>(&'a self) -> &'a f64 {
        let &NonNan(ref v) = self;
        v
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for NonNan {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let NonNan(v) = *self;
        v.fmt(f)
    }
}

impl FromStr for NonNan {
    type Err = ParseNonNanError;

    fn from_str(s: &str) -> Result<NonNan, ParseNonNanError> {
        use super::error::ParseNonNanError::*;

        match f64::from_str(s) {
            Err(e)              => Err(ParseError(e)),
            Ok(v) if v.is_nan() => Err(ValueIsNaN),
            Ok(v)               => Ok(NonNan(v)),
        }
    }
}
