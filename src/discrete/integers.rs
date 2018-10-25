use rand::Rng;
use {BoundedSpace, Space, Card, discrete::Naturals};

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

/// Type representing the set of non-negative integers, Z(≥0).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NonNegativeIntegers;

impl Space for NonNegativeIntegers {
    type Value = u64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }

    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> u64 { unimplemented!() }
}

impl BoundedSpace for NonNegativeIntegers {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<u64> { Some(0) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, _: Self::BoundValue) -> bool { true }
}

/// Type representing the set of positive integers, Z(>0).
///
/// This type is equivalent to the set of natural numbers.
pub type PositiveIntegers = Naturals;
