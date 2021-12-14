use crate::prelude::*;
use std::fmt;

/// Type representing the set of unsigned 64-bit integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Naturals;

impl Space for Naturals {
    type Value = u64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, _: &u64) -> bool { true }
}

impl OrderedSpace for Naturals {
    fn min(&self) -> Option<u64> { Some(0) }

    fn max(&self) -> Option<u64> { None }
}

impl_union_intersect!(Naturals, Naturals);

impl fmt::Display for Naturals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(≥0)")
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
    fn test_dim() {
        let d = Naturals;

        assert_eq!(d.dim(), Dim::one());
    }

    #[test]
    fn test_card() {
        let d = Naturals;

        assert_eq!(d.card(), Card::Infinite);
    }

    #[test]
    fn test_bounds() {
        let d = Naturals;

        assert_eq!(d.inf().unwrap(), 0);
        assert!(d.sup().is_none());

        assert!(d.contains(&0));
        assert!(d.contains(&1));
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Naturals;

        assert_tokens(&d, &[Token::UnitStruct { name: "Naturals" }]);
    }
}
