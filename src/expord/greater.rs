use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::num::ParseFloatError;


macro_rules! generate_impl {
    ($tyname:ident, $underlying:ty) => {
        /// This is a newtype that wraps an internal floating-point number, providing an
        /// implementation of `Ord` such that `NaN` will be ordered greater than all other floats.
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
                    (true, false)   => Ordering::Greater,
                    (true, true)    => Ordering::Equal,
                    (false, true)   => Ordering::Less,
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

generate_impl!(GreaterNan32, f32);
generate_impl!(GreaterNan64, f64);


impl FromStr for GreaterNan32 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<GreaterNan32, ParseFloatError> {
        match f32::from_str(s) {
            Err(e) => Err(e),
            Ok(v)  => Ok(GreaterNan32(v)),
        }
    }
}


impl FromStr for GreaterNan64 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<GreaterNan64, ParseFloatError> {
        match f64::from_str(s) {
            Err(e) => Err(e),
            Ok(v)  => Ok(GreaterNan64(v)),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{GreaterNan32, GreaterNan64};

    #[test]
    fn test_basics_32() {
        use ::std::f32::{NAN, INFINITY};

        assert!(GreaterNan32::new(NAN) > GreaterNan32::new(1.0));
        assert!(GreaterNan32::new(NAN) > GreaterNan32::new(INFINITY));
        assert!(GreaterNan32::new(2.0) > GreaterNan32::new(1.0));

        assert_eq!(GreaterNan32::new(NAN).cmp(&GreaterNan32::new(NAN)), Ordering::Equal);
    }

    #[test]
    fn test_basics_64() {
        use ::std::f64::{NAN, INFINITY};

        assert!(GreaterNan64::new(NAN) > GreaterNan64::new(1.0));
        assert!(GreaterNan64::new(NAN) > GreaterNan64::new(INFINITY));
        assert!(GreaterNan64::new(2.0) > GreaterNan64::new(1.0));

        assert_eq!(GreaterNan64::new(NAN).cmp(&GreaterNan64::new(NAN)), Ordering::Equal);
    }
}
