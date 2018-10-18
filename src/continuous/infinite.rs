use continuous::Continuous;
use core::{Space, Card, Surjection};
use rand::Rng;

/// An infinite dimension.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Infinite;

impl Infinite {
    pub fn bounded(self, lb: f64, ub: f64) -> Continuous { Continuous::new(lb, ub) }
}

impl Space for Infinite {
    type Value = f64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> f64 { unimplemented!() }
}

impl Surjection<f64, f64> for Infinite {
    fn map(&self, val: f64) -> f64 { val }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_bounded() {
        let d = Infinite;

        assert_eq!(d.bounded(0.0, 1.0), Continuous::new(0.0, 1.0));
    }

    #[test]
    fn test_card() {
        let d = Infinite;

        assert_eq!(d.card(), Card::Infinite);

        assert_tokens(&d, &[Token::UnitStruct { name: "Infinite" }]);
    }

    #[test]
    #[should_panic]
    fn test_sampling() {
        let d = Infinite;
        let mut rng = thread_rng();

        let _ = d.sample(&mut rng);
    }

    #[test]
    fn test_surjection() {
        let d = Infinite;
        let mut rng = thread_rng();

        for _ in 0..10 {
            let v = rng.gen::<f64>();

            assert_eq!(d.map(v), v);
        }
    }

    #[test]
    fn test_serialisation() {
        let d = Infinite;

        assert_tokens(&d, &[Token::UnitStruct { name: "Infinite" }]);
    }
}
