use Surjection;
use rand::Rng;
use super::*;

/// A binary dimension.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Binary;

impl Binary {
    pub fn new() -> Binary {
        Binary
    }
}

impl Dimension for Binary {
    type Value = bool;

    fn span(&self) -> Span {
        Span::Finite(2)
    }

    fn sample(&self, rng: &mut ThreadRng) -> bool {
        rng.gen()
    }
}

impl BoundedDimension for Binary {
    type ValueBound = bool;

    fn lb(&self) -> &bool { &false }

    fn ub(&self) -> &bool { &true }

    fn contains(&self, _: Self::Value) -> bool { true }

    fn is_infinite(&self) -> bool {
        false
    }
}

impl FiniteDimension for Binary {
    fn range(&self) -> Range<Self::Value> {
        false..true
    }
}

impl Surjection<f64, bool> for Binary {
    fn map(&self, val: f64) -> Self::Value {
        val >= 0.0
    }
}


#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_binary() {
        let d = Binary::new();

        assert_eq!(d.span(), Span::Finite(2));

        assert_tokens(&d, &[Token::UnitStruct { name: "Binary" }]);
    }
}
