use crate::prelude::*;
use std::fmt;

/// Type representing the set of natural numbers, N.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Naturals;

impl Space for Naturals {
    type Value = u64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for Naturals {
    fn inf(&self) -> Option<u64> { Some(1) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, val: u64) -> bool { val > 0 }
}

impl Surjection<u64, u64> for Naturals {
    fn map_onto(&self, val: u64) -> u64 { val.max(1) }
}

impl_union_intersect!(Naturals, Naturals);

impl fmt::Display for Naturals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2115}")
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

        assert_eq!(d.inf().unwrap(), 1);
        assert!(d.sup().is_none());

        assert!(d.contains(1));
        assert!(!d.contains(0));
    }

    #[test]
    fn test_surjection() {
        let d = Naturals;

        assert_eq!(d.map_onto(0), 1);
        assert_eq!(d.map_onto(1), 1);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Naturals;

        assert_tokens(&d, &[Token::UnitStruct { name: "Naturals" }]);
    }
}
