use super::*;

/// An infinite dimension.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Infinite;

impl Infinite {
    pub fn bounded(self, lb: f64, ub: f64) -> Continuous {
        Continuous::new(lb, ub)
    }
}

impl Dimension for Infinite {
    type Value = f64;

    fn sample(&self, _: &mut ThreadRng) -> f64 {
        unimplemented!()
    }

    fn convert(&self, val: f64) -> Self::Value {
        val
    }

    fn span(&self) -> Span {
        Span::Infinite
    }
}

impl<D: BoundedDimension> From<D> for Infinite where D::Value: PartialOrd {
    fn from(d: D) -> Infinite {
        if d.is_infinite() {
            Infinite
        } else {
            panic!("Upper or lower bound must be infinite for a valid conversion.")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite() {
        let d = Infinite;

        assert_eq!(d.span(), Span::Infinite);

        assert_tokens(&d, &[Token::UnitStruct { name: "Infinite" }]);
    }

    #[test]
    #[should_panic]
    fn test_infinite_sample() {
        let d = Infinite;
        let mut rng = thread_rng();

        let _ = d.sample(&mut rng);
    }
}
