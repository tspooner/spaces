use core::*;
use std::{cmp, fmt, ops::Range};

/// Type representing a finite, ordinal set of values.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Ordinal {
    size: usize,
}

impl Ordinal {
    pub fn new(size: usize) -> Ordinal {
        Ordinal {
            size: size,
        }
    }
}

impl From<usize> for Ordinal {
    fn from(t: usize) -> Ordinal {
        Ordinal::new(t)
    }
}

impl Space for Ordinal {
    type Value = usize;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Finite(self.size) }
}

impl BoundedSpace for Ordinal {
    type BoundValue = usize;

    fn inf(&self) -> Option<usize> { Some(0) }

    fn sup(&self) -> Option<usize> { Some(self.size - 1) }

    fn contains(&self, val: Self::Value) -> bool { val < self.size }
}

impl FiniteSpace for Ordinal {
    fn range(&self) -> Range<Self::Value> { 0..self.size }
}

impl Enclose for Ordinal {
    fn enclose(self, other: &Ordinal) -> Ordinal {
        Ordinal::new(self.size.max(other.size))
    }
}

impl Surjection<usize, usize> for Ordinal {
    fn map(&self, val: usize) -> usize { val as usize }
}

impl cmp::PartialEq for Ordinal {
    fn eq(&self, other: &Ordinal) -> bool { self.size.eq(&other.size) }
}

impl fmt::Debug for Ordinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Ordinal")
            .field("size", &self.size)
            .finish()
    }
}

impl fmt::Display for Ordinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[0..{}]", self.size-1)
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use super::*;

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

            assert!(d.contains(0));
            assert!(d.contains(size - 1));
            assert!(!d.contains(size));
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

        assert_eq!(d.map(0), 0);
        assert_eq!(d.map(1), 1);
        assert_eq!(d.map(2), 2);
        assert_eq!(d.map(3), 3);
        assert_eq!(d.map(4), 4);
        assert_eq!(d.map(5), 5);
        assert_eq!(d.map(6), 6);
        assert_eq!(d.map(7), 7);
        assert_eq!(d.map(8), 8);
        assert_eq!(d.map(9), 9);
    }

    #[test]
    fn test_serialisation() {
        fn check(size: usize) {
            let d = Ordinal::new(size);

            assert_tokens(
                &d,
                &[
                    Token::Struct {
                        name: "Ordinal",
                        len: 1,
                    },
                    Token::Str("size"),
                    Token::U64(size as u64),
                    Token::StructEnd,
                ],
            );
        }

        check(5);
        check(10);
        check(100);
    }
}
