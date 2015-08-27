use std::error;
use std::num::ParseFloatError;
use std::fmt;


/// ParseNonNanError is the error returned when parsing a `NonNan` from a string.  It will either
/// be any underlying error from the floating-point parsing code, or an indicator that the parsed
/// value is `NaN`.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseNonNanError {
    /// The value that was parsed is NaN.
    ValueIsNaN,

    /// There was an error parsing the underlying float.
    ParseError(ParseFloatError),
}

impl fmt::Display for ParseNonNanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::ParseNonNanError::*;

        match *self {
            ValueIsNaN        => write!(f, "value is NaN"),
            ParseError(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for ParseNonNanError {
    fn description(&self) -> &str {
        use self::ParseNonNanError::*;

        match *self {
            ValueIsNaN    => "value is NaN",
            ParseError(_) => "parse error",
        }
    }

    fn cause<'a>(&'a self) -> Option<&'a error::Error> {
        use self::ParseNonNanError::*;

        match *self {
            ValueIsNaN        => None,
            ParseError(ref e) => Some(e),
        }
    }
}
