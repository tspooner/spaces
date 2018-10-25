use rand::Rng;
use {BoundedSpace, Space, Card};

/// Type representing the set of integers, Z.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Integers;

impl Space for Integers {
    type Value = i64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> i64 { unimplemented!() }
}

impl BoundedSpace for Integers {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<i64> { None }

    fn sup(&self) -> Option<i64> { None }

    fn contains(&self, _: Self::BoundValue) -> bool { true }
}

/// Type representing the set of non-zero integers, Z*.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NonZeroIntegers;

impl Space for NonZeroIntegers {
    type Value = i64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> i64 { unimplemented!() }
}

impl BoundedSpace for NonZeroIntegers {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<i64> { None }

    fn sup(&self) -> Option<i64> { None }

    fn contains(&self, val: Self::BoundValue) -> bool { val != 0 }
}
