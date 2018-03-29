use {Space, BoundedSpace, FiniteSpace, Surjection, Span};

use rand::{ThreadRng, Rng};
use std::ops::Range;

/// A binary dimension.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Binary;

impl Binary {
    pub fn new() -> Binary {
        Binary
    }
}

impl Space for Binary {
    type Value = bool;

    fn dim(&self) -> usize { 1 }

    fn span(&self) -> Span { Span::Finite(2) }

    fn sample(&self, rng: &mut ThreadRng) -> bool { rng.gen() }
}

impl BoundedSpace for Binary {
    type BoundValue = bool;

    fn lb(&self) -> &bool { &false }

    fn ub(&self) -> &bool { &true }

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


#[cfg(test)]
mod tests {
    extern crate serde_test;

    use rand::thread_rng;
    use self::serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_span() {
        let d = Binary::new();

        assert_eq!(d.span(), Span::Finite(2));
    }

    #[test]
    fn test_sampling() {
        let d = Binary::new();
        let mut rng = thread_rng();

        for _ in 0..100 {
            let s = d.sample(&mut rng);

            assert!(s == false || s == true);
            assert!(d.contains(s));
        }
    }

    #[test]
    fn test_bounds() {
        let d = Binary::new();

        assert_eq!(d.lb(), &false);
        assert_eq!(d.ub(), &true);

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
