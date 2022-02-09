use crate::prelude::*;
use std::ops::Range;

impl Space for Range<usize> {
    type Value = usize;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card {
        // TODO: when the Step trait drops, we can replace this with a direct call to
        // Step::steps_between.
        //      See https://github.com/rust-lang/rust/issues/42168
        Card::Finite(self.len())
    }

    fn contains(&self, val: &usize) -> bool { Range::contains(&self, val) }
}

impl OrderedSpace for Range<usize> {
    fn min(&self) -> Option<usize> { Some(self.start) }

    fn max(&self) -> Option<usize> { Some(self.end - 1) }
}

impl FiniteSpace for Range<usize> {
    fn to_ordinal(&self) -> Range<Self::Value> { self.clone() }
}

impl Union for Range<usize> {
    fn union(self, other: &Range<usize>) -> Range<usize> {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end)
        }
    }
}

impl Intersect for Range<usize> {
    fn intersect(self, other: &Range<usize>) -> Range<usize> {
        Range {
            start: self.start.max(other.start),
            end: self.end.min(other.end)
        }
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
            let d = 0..size;

            assert_eq!(d.card(), Card::Finite(size));
        }

        check(5);
        check(10);
        check(100);
    }

    #[test]
    fn test_bounds() {
        fn check(size: usize) {
            let d = 0..size;

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
    fn test_to_ordinal() {
        assert_eq!((0..1).to_ordinal(), 0..1);
        assert_eq!((0..5).to_ordinal(), 0..5);
        assert_eq!((0..10).to_ordinal(), 0..10);
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
