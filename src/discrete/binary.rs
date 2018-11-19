use core::{BoundedSpace, FiniteSpace, Space, Card, Surjection};
use std::{
    fmt,
    ops::Range,
};

/// Type representing binary values.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Binary;

impl Binary {
    pub fn new() -> Binary { Binary }
}

impl Space for Binary {
    type Value = bool;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Finite(2) }
}

impl BoundedSpace for Binary {
    type BoundValue = bool;

    fn inf(&self) -> Option<bool> { Some(false) }

    fn sup(&self) -> Option<bool> { Some(true) }

    fn contains(&self, _: Self::Value) -> bool { true }
}

impl FiniteSpace for Binary {
    fn range(&self) -> Range<Self::Value> { false..true }
}

impl Surjection<bool, bool> for Binary {
    fn map(&self, val: bool) -> bool { val }
}

impl Surjection<f64, bool> for Binary {
    fn map(&self, val: f64) -> bool { val > 0.0 }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{0, 1}}")
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use super::*;

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

        assert_eq!(d.map(true), true);
        assert_eq!(d.map(false), false);

        assert_eq!(d.map(1.0), true);
        assert_eq!(d.map(0.0), false);
    }

    #[test]
    fn test_serialisation() {
        let d = Binary::new();

        assert_tokens(&d, &[Token::UnitStruct { name: "Binary" }]);
    }
}
