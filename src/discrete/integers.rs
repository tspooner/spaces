use crate::prelude::*;
use num_traits::{PrimInt, Signed, Unsigned};
use std::{fmt, marker::PhantomData};

/// Type representing the set of signed integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Integers<V>(PhantomData<V>);

impl<V: PrimInt + Signed> Integers<V> {
    pub fn new() -> Self { Integers(PhantomData) }
}

impl<V: PrimInt + Signed> Space for Integers<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, _: &V) -> bool { true }
}

impl<V: PrimInt + Signed> OrderedSpace for Integers<V> {}

impl<V: PrimInt + Signed> Union for Integers<V> {
    fn union(self, _: &Integers<V>) -> Self { self }
}

impl<V: PrimInt + Signed> Intersect for Integers<V> {
    fn intersect(self, _: &Integers<V>) -> Self { self }
}

impl<V> fmt::Display for Integers<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}")
    }
}

/// Type representing the set of non-zero signed integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonZeroIntegers<V>(PhantomData<V>);

impl<V: PrimInt + Signed> NonZeroIntegers<V> {
    pub fn new() -> Self { NonZeroIntegers(PhantomData) }
}

impl<V: PrimInt + Signed> Space for NonZeroIntegers<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, val: &V) -> bool { !val.is_zero() }
}

impl<V: PrimInt + Signed> OrderedSpace for NonZeroIntegers<V> {}

impl<V: PrimInt + Signed> Union for NonZeroIntegers<V> {
    fn union(self, _: &NonZeroIntegers<V>) -> Self { self }
}

impl<V: PrimInt + Signed> Intersect for NonZeroIntegers<V> {
    fn intersect(self, _: &NonZeroIntegers<V>) -> Self { self }
}

impl<V> fmt::Display for NonZeroIntegers<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(!=0)")
    }
}

/// Type representing the set of unsigned integers.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonNegativeIntegers<V>(PhantomData<V>);

impl<V: PrimInt + Unsigned> NonNegativeIntegers<V> {
    pub fn new() -> Self { NonNegativeIntegers(PhantomData) }
}

impl<V: PrimInt + Unsigned> Space for NonNegativeIntegers<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, _: &V) -> bool { true }
}

impl<V: PrimInt + Unsigned> OrderedSpace for NonNegativeIntegers<V> {
    fn min(&self) -> Option<V> { Some(V::zero()) }
}

impl<V: PrimInt + Unsigned> Union for NonNegativeIntegers<V> {
    fn union(self, _: &NonNegativeIntegers<V>) -> Self { self }
}

impl<V: PrimInt + Unsigned> Intersect for NonNegativeIntegers<V> {
    fn intersect(self, _: &NonNegativeIntegers<V>) -> Self { self }
}

impl<V> fmt::Display for NonNegativeIntegers<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2124}(≥0)")
    }
}
