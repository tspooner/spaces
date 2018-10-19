use continuous::Interval;
use core::{Space, Card, Surjection};
use rand::Rng;

/// An infinite dimension.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Reals;

impl Reals {
    pub fn bounded(self, lb: f64, ub: f64) -> Interval { Interval::closed(lb, ub) }

    pub fn left_bounded(self, lb: f64) -> Interval { Interval::left_closed(lb) }

    pub fn right_bounded(self, ub: f64) -> Interval { Interval::right_closed(ub) }
}

impl Space for Reals {
    type Value = f64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> f64 { unimplemented!() }
}

impl Surjection<f64, f64> for Reals {
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
        let d = Reals;

        assert_eq!(d.bounded(0.0, 1.0), Interval::closed(0.0, 1.0));
    }

    #[test]
    fn test_card() {
        let d = Reals;

        assert_eq!(d.card(), Card::Infinite);

        assert_tokens(&d, &[Token::UnitStruct { name: "Reals" }]);
    }

    #[test]
    #[should_panic]
    fn test_sampling() {
        let d = Reals;
        let mut rng = thread_rng();

        let _ = d.sample(&mut rng);
    }

    #[test]
    fn test_surjection() {
        let d = Reals;
        let mut rng = thread_rng();

        for _ in 0..10 {
            let v = rng.gen::<f64>();

            assert_eq!(d.map(v), v);
        }
    }

    #[test]
    fn test_serialisation() {
        let d = Reals;

        assert_tokens(&d, &[Token::UnitStruct { name: "Reals" }]);
    }
}
