use Surjection;
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

    fn span(&self) -> Span {
        Span::Infinite
    }

    fn sample(&self, _: &mut ThreadRng) -> f64 {
        unimplemented!()
    }
}

impl Surjection<f64, f64> for Infinite {
    fn map(&self, val: f64) -> f64 {
        val
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
    use rand::thread_rng;
    use serde_test::{assert_tokens, Token};
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
