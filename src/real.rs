//! Real spaces module.
use crate::prelude::*;
use num_traits::real::Real;
use std::{fmt, marker::PhantomData};

/// Type representing the set of all real numbers.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Reals<V>(PhantomData<V>);

impl<V: Real> Reals<V> {
    pub fn new() -> Reals<V> { Reals(PhantomData) }
}

impl<V: Real> Space for Reals<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, _: &V) -> bool { true }
}

impl<V: Real> OrderedSpace for Reals<V> {}

impl<V: Real> Union for Reals<V> {
    fn union(self, _: &Reals<V>) -> Self { self }
}

impl<V: Real> Intersect for Reals<V> {
    fn intersect(self, _: &Reals<V>) -> Self { self }
}

impl<V> fmt::Display for Reals<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "\u{211d}") }
}

/// Type representing the set of non-negative real numbers, R(≥0).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonNegativeReals<V>(PhantomData<V>);

impl<V: Real> NonNegativeReals<V> {
    pub fn new() -> Self { NonNegativeReals(PhantomData) }
}

impl<V: Real> Space for NonNegativeReals<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, val: &V) -> bool { val.is_sign_positive() }
}

impl<V: Real> OrderedSpace for NonNegativeReals<V> {
    fn min(&self) -> Option<V> { Some(V::zero()) }
}

impl<V: Real> Union for NonNegativeReals<V> {
    fn union(self, _: &NonNegativeReals<V>) -> Self { self }
}

impl<V: Real> Intersect for NonNegativeReals<V> {
    fn intersect(self, _: &NonNegativeReals<V>) -> Self { self }
}

impl<V> fmt::Display for NonNegativeReals<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "\u{211d}(>0)") }
}

/// Type representing the set of strictly positive real numbers, R(>0).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PositiveReals<V>(PhantomData<V>);

impl<V: Real> PositiveReals<V> {
    pub fn new() -> Self { PositiveReals(PhantomData) }
}

impl<V: Real> Space for PositiveReals<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, val: &V) -> bool { *val > V::zero() }
}

impl<V: Real> OrderedSpace for PositiveReals<V> {
    fn inf(&self) -> Option<V> { Some(V::zero()) }
}

impl<V: Real> Union for PositiveReals<V> {
    fn union(self, _: &PositiveReals<V>) -> Self { self }
}

impl<V: Real> Intersect for PositiveReals<V> {
    fn intersect(self, _: &PositiveReals<V>) -> Self { self }
}

impl<V> fmt::Display for PositiveReals<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "\u{211d}(≥0)") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    extern crate serde_test;
    #[cfg(feature = "serialize")]
    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn test_card() {
        let d = Reals::<f64>::new();

        assert_eq!(d.card(), Card::Infinite);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Reals::<f64>::new();

        assert_tokens(&d, &[
            Token::NewtypeStruct { name: "Reals", },
            Token::UnitStruct { name: "PhantomData", },
        ]);
    }
}
