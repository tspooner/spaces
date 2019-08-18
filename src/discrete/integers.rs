use crate::{
    Space, BoundedSpace,
    core::*,
    discrete::Naturals,
};
use std::fmt;

/// Type representing the set of integers, Z.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Integers;

impl Space for Integers {
    type Value = i64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for Integers {
    fn inf(&self) -> Option<i64> { None }

    fn sup(&self) -> Option<i64> { None }

    fn contains(&self, _: i64) -> bool { true }
}

impl_auto_union!(Integers, Integers);

impl fmt::Display for Integers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}")
    }
}

/// Type representing the set of non-zero integers, Z*.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonZeroIntegers;

impl Space for NonZeroIntegers {
    type Value = i64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for NonZeroIntegers {
    fn inf(&self) -> Option<i64> { None }

    fn sup(&self) -> Option<i64> { None }

    fn contains(&self, val: i64) -> bool { val != 0 }
}

impl_auto_union!(NonZeroIntegers, NonZeroIntegers);

impl fmt::Display for NonZeroIntegers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(>0)")
    }
}

/// Type representing the set of non-negative integers, Z(≥0).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonNegativeIntegers;

impl Space for NonNegativeIntegers {
    type Value = u64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for NonNegativeIntegers {
    fn inf(&self) -> Option<u64> { Some(0) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, _: u64) -> bool { true }
}

impl_auto_union!(NonNegativeIntegers, NonNegativeIntegers);

impl fmt::Display for NonNegativeIntegers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(≥0)")
    }
}

/// Type representing the set of positive integers, Z(>0).
///
/// This type is equivalent to the set of natural numbers.
pub type PositiveIntegers = Naturals;
