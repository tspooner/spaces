use rand::Rng;
use {BoundedSpace, Space, Card};

/// The set of all natural numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Natural;

impl Space for Natural {
    type Value = u64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> u64 { unimplemented!() }
}

impl BoundedSpace for Natural {
    type BoundValue = Self::Value;

    fn lb(&self) -> &u64 { &0 }

    fn ub(&self) -> &u64 { unimplemented!() }

    fn contains(&self, val: Self::BoundValue) -> bool { val >= 0 }
}
