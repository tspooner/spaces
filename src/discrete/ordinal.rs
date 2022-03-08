use crate::prelude::*;
use num_traits::PrimInt;
use std::ops::Range;

pub type Ordinal<V> = Range<V>;

impl<V: PrimInt> Space for Range<V> where Self: ExactSizeIterator {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card {
        // TODO: when the Step trait drops, we can replace this with a direct call to
        // Step::steps_between.
        //      See https://github.com/rust-lang/rust/issues/42168
        Card::Finite(self.len())
    }

    fn contains(&self, val: &V) -> bool { Range::contains(&self, val) }
}

impl<V: PrimInt> OrderedSpace for Range<V> where Self: ExactSizeIterator {
    fn min(&self) -> Option<V> { Some(self.start) }

    fn max(&self) -> Option<V> { Some(self.end - V::one()) }
}

impl<V: PrimInt> FiniteSpace for Range<V> where Self: ExactSizeIterator {
    fn to_ordinal(&self) -> Range<usize> {
        Range { start: 0, end: self.len(), }
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
        fn check<const N: u64>() {
            let d = 0..N;

            assert_tokens(&d, &[
                Token::Struct { name: "Range", len: 2, },
                Token::Str("start"),
                Token::U64(0),
                Token::Str("end"),
                Token::U64(N),
                Token::StructEnd,
            ]);
        }

        check::<5>();
        check::<10>();
        check::<100>();
    }
}
