//! Newtype wrappers for floats that are not `NaN`.

/// Error definitions.
pub mod error;

/// Newtype implementation for f64.
pub mod f64;


#[cfg(test)]
mod test {
    use std::error;
    use std::str::FromStr;

    use super::f64::NonNan;
    use super::error::ParseNonNanError;


    #[test]
    fn test_new() {
        use std::f64::NAN;

        assert!(NonNan::new(NAN).is_none());
        assert!(NonNan::new(1.23).is_some());
    }

    #[test]
    fn test_ordering() {
        assert!(NonNan::new(1.0).unwrap() < NonNan::new(2.0).unwrap());
    }

    #[test]
    fn test_deref() {
        use std::f64::INFINITY;

        let v = NonNan::new(INFINITY).unwrap();

        // This derefs to the f64 implementation.
        assert!(v.is_infinite());
    }

    #[test]
    fn test_from_string() {
        use super::error::ParseNonNanError::*;

        match NonNan::from_str("1.0") {
            Ok(v)  => assert_eq!(v, NonNan::new(1.0).unwrap()),
            Err(e) => panic!("unexpected error: {:?}", e),
        };

        match NonNan::from_str("NaN") {
            Ok(v)           => panic!("unexpected success: {:?}", v),
            Err(ValueIsNaN) => {},
            Err(e)          => panic!("unexpected error: {:?}", e),
        };

        match NonNan::from_str("foo") {
            Ok(v)              => panic!("unexpected success: {:?}", v),
            Err(ParseError(_)) => {},
            Err(e)             => panic!("unexpected error: {:?}", e),
        };


        fn assert_error<T: error::Error>(_v: T) {}
        assert_error(ParseNonNanError::ValueIsNaN);
    }
}
