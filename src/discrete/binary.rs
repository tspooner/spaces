use crate::prelude::*;
use std::{fmt, ops::Range};

/// Type representing binary (base-2) values.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Binary;

impl Binary {
    pub fn new() -> Binary { Binary }
}

impl Space for Binary {
    type Value = bool;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Finite(2) }
}

impl BoundedSpace for Binary {
    fn inf(&self) -> Option<bool> { Some(false) }

    fn sup(&self) -> Option<bool> { Some(true) }

    fn contains(&self, _: bool) -> bool { true }
}

impl FiniteSpace for Binary {
    fn range(&self) -> Range<Self::Value> { false..true }
}

impl_auto_union!(Binary, Binary);

impl Surjection<bool, bool> for Binary {
    fn map_onto(&self, val: bool) -> bool { val }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{0, 1}}")
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
        let d = Binary::new();

        assert_eq!(d.card(), Card::Finite(2));
    }

    #[test]
    fn test_bounds() {
        let d = Binary::new();

        assert_eq!(d.inf().unwrap(), false);
        assert_eq!(d.sup().unwrap(), true);

        assert!(d.contains(false));
        assert!(d.contains(true));
    }

    #[test]
    fn test_range() {
        let d = Binary::new();
        let r = d.range();

        assert!(r == (false..true) || r == (true..false));
    }

    #[test]
    fn test_surjection() {
        let d = Binary::new();

        assert_eq!(d.map_onto(true), true);
        assert_eq!(d.map_onto(false), false);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Binary::new();

        assert_tokens(&d, &[Token::UnitStruct { name: "Binary" }]);
    }
}
