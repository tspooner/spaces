use crate::prelude::*;
use std::fmt;

/// Type representing the set of 64-bit integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Integers;

impl Space for Integers {
    const DIM: usize = 1;

    type Value = i64;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, _: &i64) -> bool { true }
}

impl OrderedSpace for Integers {}

impl_union_intersect!(Integers, Integers);

impl fmt::Display for Integers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}")
    }
}

/// Type representing the set of non-zero 64-bit integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonZeroIntegers;

impl Space for NonZeroIntegers {
    const DIM: usize = 1;

    type Value = i64;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, val: &i64) -> bool { *val != 0 }
}

impl OrderedSpace for NonZeroIntegers {}

impl_union_intersect!(NonZeroIntegers, NonZeroIntegers);

impl fmt::Display for NonZeroIntegers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(!=0)")
    }
}

/// Type representing the set of unsigned 64-bit integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonNegativeIntegers;

impl Space for NonNegativeIntegers {
    const DIM: usize = 1;

    type Value = i64;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, _: &i64) -> bool { true }
}

impl OrderedSpace for NonNegativeIntegers {
    fn min(&self) -> Option<i64> { Some(0) }
}

impl_union_intersect!(NonNegativeIntegers, NonNegativeIntegers);

impl fmt::Display for NonNegativeIntegers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(≥0)")
    }
}
