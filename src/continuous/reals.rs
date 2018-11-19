use continuous::Interval;
use core::{BoundedSpace, Space, Card, Surjection};
use rand::Rng;
use std::fmt;

/// Type representing the set of all real numbers.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Reals;

impl Reals {
    pub fn bounded(self, lb: f64, ub: f64) -> Interval { Interval::bounded(lb, ub) }

    pub fn left_bounded(self, lb: f64) -> Interval { Interval::left_bounded(lb) }

    pub fn right_bounded(self, ub: f64) -> Interval { Interval::right_bounded(ub) }
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

impl fmt::Display for Reals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{211d}")
    }
}

/// Type representing the set of non-negative real numbers, R(≥0).
#[derive(Clone, Copy, Serialize)]
pub struct NonNegativeReals;

impl Space for NonNegativeReals {
    type Value = f64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> f64 {
        unimplemented!()
    }
}

impl BoundedSpace for NonNegativeReals {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<f64> { Some(0.0) }

    fn sup(&self) -> Option<f64> { None }

    fn contains(&self, val: Self::BoundValue) -> bool { val >= 0.0 }
}

impl Surjection<f64, f64> for NonNegativeReals {
    fn map(&self, val: f64) -> f64 { val.max(0.0) }
}

impl fmt::Display for NonNegativeReals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{211d}(>0)")
    }
}

/// Type representing the set of strictly positive real numbers, R(>0).
#[derive(Clone, Copy, Serialize)]
pub struct PositiveReals;

impl Space for PositiveReals {
    type Value = f64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> f64 {
        unimplemented!()
    }
}

impl BoundedSpace for PositiveReals {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<f64> { Some(1e-5) }

    fn sup(&self) -> Option<f64> { None }

    fn contains(&self, val: Self::BoundValue) -> bool { val > 0.0 }
}

impl Surjection<f64, f64> for PositiveReals {
    fn map(&self, val: f64) -> f64 { val.max(1e-5) }
}

impl fmt::Display for PositiveReals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{211d}(≥0)")
    }
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

        assert_eq!(d.bounded(0.0, 1.0), Interval::bounded(0.0, 1.0));
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
