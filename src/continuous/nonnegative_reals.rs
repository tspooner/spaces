use core::{BoundedSpace, Space, Card, Surjection};
use rand::Rng;

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
