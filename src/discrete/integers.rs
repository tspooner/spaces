use core::{BoundedSpace, Space, Card};
use discrete::Naturals;
use std::fmt;

/// Type representing the set of integers, Z.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Integers;

impl Space for Integers {
    type Value = i64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for Integers {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<i64> { None }

    fn sup(&self) -> Option<i64> { None }

    fn contains(&self, _: Self::BoundValue) -> bool { true }
}

impl fmt::Display for Integers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}")
    }
}

/// Type representing the set of non-zero integers, Z*.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NonZeroIntegers;

impl Space for NonZeroIntegers {
    type Value = i64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for NonZeroIntegers {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<i64> { None }

    fn sup(&self) -> Option<i64> { None }

    fn contains(&self, val: Self::BoundValue) -> bool { val != 0 }
}

impl fmt::Display for NonZeroIntegers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(>0)")
    }
}

/// Type representing the set of non-negative integers, Z(≥0).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NonNegativeIntegers;

impl Space for NonNegativeIntegers {
    type Value = u64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for NonNegativeIntegers {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<u64> { Some(0) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, _: Self::BoundValue) -> bool { true }
}

impl fmt::Display for NonNegativeIntegers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(≥0)")
    }
}

/// Type representing the set of positive integers, Z(>0).
///
/// This type is equivalent to the set of natural numbers.
pub type PositiveIntegers = Naturals;
