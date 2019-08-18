use crate::{
    Space, BoundedSpace,
    core::*,
};
use std::fmt;

/// Type representing the set of natural numbers, N.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Naturals;

impl Space for Naturals {
    type Value = u64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for Naturals {
    fn inf(&self) -> Option<u64> { Some(1) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, val: u64) -> bool { val > 0 }
}

impl_auto_union!(Naturals, Naturals);

impl fmt::Display for Naturals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2115}")
    }
}
