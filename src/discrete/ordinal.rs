use crate::prelude::*;
use std::{cmp, fmt, ops::Range};

/// Type representing a finite, ordinal set of values.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Ordinal(usize);

impl Ordinal {
    pub fn new(size: usize) -> Ordinal {
        Ordinal(size)
    }
}

impl From<usize> for Ordinal {
    fn from(t: usize) -> Ordinal {
        Ordinal::new(t)
    }
}

impl Space for Ordinal {
    type Value = usize;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Finite(self.0) }

    fn contains(&self, val: &usize) -> bool { *val < self.0 }

    fn min(&self) -> Option<usize> { Some(0) }

    fn max(&self) -> Option<usize> { Some(self.0 - 1) }
}

impl FiniteSpace for Ordinal {
    fn range(&self) -> Range<Self::Value> { 0..self.0 }
}

impl Union for Ordinal {
    fn union(self, other: &Ordinal) -> Ordinal {
        Ordinal::new(self.0.max(other.0))
    }
}

impl Intersection for Ordinal {
    fn intersect(self, other: &Ordinal) -> Ordinal {
        Ordinal::new(self.0.min(other.0))
    }
}

impl Project<usize, usize> for Ordinal {
    fn project(&self, val: usize) -> usize { val as usize }
}

impl cmp::PartialEq for Ordinal {
    fn eq(&self, other: &Ordinal) -> bool { self.0.eq(&other.0) }
}

impl fmt::Display for Ordinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[0..{}]", self.sup().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    extern crate serde_test;
    #[cfg(feature = "serialize")]
    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn test_card() {
        fn check(size: usize) {
            let d = Ordinal::new(size);

            assert_eq!(d.card(), Card::Finite(size));
        }

        check(5);
        check(10);
        check(100);
    }

    #[test]
    fn test_bounds() {
        fn check(size: usize) {
            let d = Ordinal::new(size);

            assert_eq!(d.inf().unwrap(), 0);
            assert_eq!(d.sup().unwrap(), size - 1);

            assert!(d.contains(&0));
            assert!(d.contains(&(size - 1)));
            assert!(!d.contains(&size));
        }

        check(5);
        check(10);
        check(100);
    }

    #[test]
    fn test_range() {
        assert_eq!(Ordinal::new(1).range(), 0..1);
        assert_eq!(Ordinal::new(5).range(), 0..5);
        assert_eq!(Ordinal::new(10).range(), 0..10);
    }

    #[test]
    fn test_surjection() {
        let d = Ordinal::new(10);

        assert_eq!(d.project(0), 0);
        assert_eq!(d.project(1), 1);
        assert_eq!(d.project(2), 2);
        assert_eq!(d.project(3), 3);
        assert_eq!(d.project(4), 4);
        assert_eq!(d.project(5), 5);
        assert_eq!(d.project(6), 6);
        assert_eq!(d.project(7), 7);
        assert_eq!(d.project(8), 8);
        assert_eq!(d.project(9), 9);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        fn check(size: usize) {
            let d = Ordinal::new(size);

            assert_tokens(
                &d,
                &[
                    Token::NewtypeStruct { name: "Ordinal", },
                    Token::U64(size as u64),
                ],
            );
        }

        check(5);
        check(10);
        check(100);
    }
}
