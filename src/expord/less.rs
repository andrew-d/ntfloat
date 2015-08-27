use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::num::ParseFloatError;


macro_rules! generate_impl {
    ($tyname:ident, $underlying:ty) => {
        /// This is a newtype that wraps an internal floating-point number, providing an
        /// implementation of `Ord` such that `NaN` will be ordered less than all other floats.
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $tyname($underlying);

        impl $tyname {
            /// Create a new instance with the given floating-point number.
            pub fn new(val: $underlying) -> $tyname {
                $tyname(val)
            }
        }

        impl Deref for $tyname {
            type Target = $underlying;

            fn deref<'a>(&'a self) -> &'a $underlying {
                let &$tyname(ref v) = self;
                v
            }
        }

        impl Eq for $tyname {}

        impl PartialOrd for $tyname {
            fn partial_cmp(&self, other: &$tyname) -> Option<Ordering> {
                let &$tyname(ref us) = self;
                let &$tyname(ref them) = other;

                let ord = match (us.is_nan(), them.is_nan()) {
                    (true, false)   => Ordering::Less,
                    (true, true)    => Ordering::Equal,
                    (false, true)   => Ordering::Greater,
                    (false, false)  => us.partial_cmp(them).unwrap(),
                };

                Some(ord)
            }
        }

        impl Ord for $tyname {
            fn cmp(&self, other: &$tyname) -> Ordering {
                // The above implementation should never return None.
                self.partial_cmp(other).unwrap()
            }
        }

        impl fmt::Display for $tyname {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                let $tyname(v) = *self;
                v.fmt(f)
            }
        }
    }
}

generate_impl!(LessNan32, f32);
generate_impl!(LessNan64, f64);


impl FromStr for LessNan32 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<LessNan32, ParseFloatError> {
        match f32::from_str(s) {
            Err(e) => Err(e),
            Ok(v)  => Ok(LessNan32(v)),
        }
    }
}


impl FromStr for LessNan64 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<LessNan64, ParseFloatError> {
        match f64::from_str(s) {
            Err(e) => Err(e),
            Ok(v)  => Ok(LessNan64(v)),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{LessNan32, LessNan64};

    #[test]
    fn test_basics_32() {
        use ::std::f32::{NAN, INFINITY};

        assert!(LessNan32::new(NAN) < LessNan32::new(1.0));
        assert!(LessNan32::new(NAN) < LessNan32::new(INFINITY));
        assert!(LessNan32::new(1.0) < LessNan32::new(2.0));

        assert_eq!(LessNan32::new(NAN).cmp(&LessNan32::new(NAN)), Ordering::Equal);
    }

    #[test]
    fn test_basics_64() {
        use ::std::f64::{NAN, INFINITY};

        assert!(LessNan64::new(NAN) < LessNan64::new(1.0));
        assert!(LessNan64::new(NAN) < LessNan64::new(INFINITY));
        assert!(LessNan64::new(1.0) < LessNan64::new(2.0));

        assert_eq!(LessNan64::new(NAN).cmp(&LessNan64::new(NAN)), Ordering::Equal);
    }
}
