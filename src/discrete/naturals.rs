use rand::Rng;
use {BoundedSpace, Space, Card};

/// Type representing the set of natural numbers, N.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Naturals;

impl Space for Naturals {
    type Value = u64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> u64 { unimplemented!() }
}

impl BoundedSpace for Naturals {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<u64> { Some(1) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, val: Self::BoundValue) -> bool { val > 0 }
}
