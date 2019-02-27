use core::*;
use std::fmt;

/// Type representing the set of natural numbers, N.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Naturals;

impl Space for Naturals {
    type Value = u64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for Naturals {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<u64> { Some(1) }

    fn sup(&self) -> Option<u64> { None }

    fn contains(&self, val: Self::BoundValue) -> bool { val > 0 }
}

impl_auto_enclose!(Naturals, Naturals);

impl fmt::Display for Naturals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2115}")
    }
}
