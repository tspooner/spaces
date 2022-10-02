//! Module for discrete scalar spaces.
use crate::{intervals, ops};
use num_traits::{PrimInt, Signed, Unsigned};

/// Build a space representing binary (base-2) values.
pub fn binary() -> Binary {
    intervals::Interval {
        left: intervals::bounds::Closed(false),
        right: intervals::bounds::Closed(true),
    }
}

pub type Binary = intervals::Closed<bool>;

/// Build a space representing the set of signed integers.
pub fn integers<V: PrimInt + Signed>() -> Integers<V> { intervals::Interval::unbounded() }

pub type Integers<V> = intervals::Unbounded<V>;

/// Build a space representing the set of non-zero signed integers.
pub fn non_zero_integers<V: PrimInt + Signed>() -> NonZeroIntegers<V> {
    let x = intervals::Interval::right_open(V::zero());
    let y = intervals::Interval::left_open(V::zero());

    ops::UnionPair(x, y)
}

pub type NonZeroIntegers<V> = ops::UnionPair<intervals::RightOpen<V>, intervals::LeftOpen<V>>;

/// Build a space representing the set of unsigned integers.
pub fn non_negative_integers<V: PrimInt>() -> NonNegativeIntegers<V> {
    intervals::Interval::left_closed(V::zero())
}

pub type NonNegativeIntegers<V> = intervals::LeftClosed<V>;

/// Build a space representing the set of unsigned integers.
pub fn positive_integers<V: PrimInt + Unsigned>() -> PositiveIntegers<V> {
    intervals::Interval::left_open(V::zero())
}

pub type PositiveIntegers<V> = intervals::LeftOpen<V>;

/// Build a space representing the set of unsigned integers.
pub fn non_positive_integers<V: PrimInt + Signed>() -> NonPositiveIntegers<V> {
    intervals::Interval::right_closed(V::zero())
}

pub type NonPositiveIntegers<V> = intervals::RightClosed<V>;

/// Build a space representing the set of unsigned integers.
pub fn negative_integers<V: PrimInt + Signed>() -> NegativeIntegers<V> {
    intervals::Interval::right_open(V::zero())
}

pub type NegativeIntegers<V> = intervals::RightOpen<V>;

/// Build a space representing the set of natural numbers.
pub fn naturals<V: PrimInt + Unsigned>() -> Naturals<V> { positive_integers() }

pub type Naturals<V> = PositiveIntegers<V>;
