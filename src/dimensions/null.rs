use super::*;

/// A null dimension.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Null;

impl Dimension for Null {
    type Value = ();

    fn sample(&self, _: &mut ThreadRng) -> () {
        ()
    }

    fn convert(&self, _: f64) -> Self::Value {
        ()
    }

    fn span(&self) -> Span {
        Span::Null
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let d = Null;
        let mut rng = thread_rng();

        assert_eq!(d.sample(&mut rng), ());
        assert_eq!(d.span(), Span::Null);

        assert_tokens(&d, &[Token::UnitStruct { name: "Null" }]);
    }
}
