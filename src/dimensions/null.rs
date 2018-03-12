use super::*;

/// A null dimension.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Null;

impl Dimension for Null {
    type Value = ();

    fn convert(&self, _: f64) -> Self::Value {
        ()
    }

    fn span(&self) -> Span {
        Span::Null
    }

    fn sample(&self, _: &mut ThreadRng) -> () {
        ()
    }
}


#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_null() {
        let d = Null;

        assert_eq!(d.span(), Span::Null);

        assert_tokens(&d, &[Token::UnitStruct { name: "Null" }]);
    }
}
