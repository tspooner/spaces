use core::{BoundedSpace, Space, Card, Surjection};

/// Type representing the set of strictly positive real numbers, R(>0).
#[derive(Clone, Copy, Serialize)]
pub struct PositiveReals;

impl Space for PositiveReals {
    type Value = f64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }
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
