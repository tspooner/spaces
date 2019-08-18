//! Real spaces module.
use crate::prelude::*;
use std::fmt;

pub type Interval = crate::Interval<f64>;

/// Type representing the set of all real numbers.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Reals;

impl Reals {
    pub fn bounded(self, lb: f64, ub: f64) -> Interval { Interval::bounded(lb, ub) }

    pub fn left_bounded(self, lb: f64) -> Interval { Interval::left_bounded(lb) }

    pub fn right_bounded(self, ub: f64) -> Interval { Interval::right_bounded(ub) }
}

impl Space for Reals {
    type Value = f64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl_union_intersect!(Reals, Reals);

impl Surjection<f64, f64> for Reals {
    fn map_onto(&self, val: f64) -> f64 { val }
}

impl fmt::Display for Reals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{211d}")
    }
}

/// Type representing the set of non-negative real numbers, R(≥0).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NonNegativeReals;

impl Space for NonNegativeReals {
    type Value = f64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for NonNegativeReals {
    fn inf(&self) -> Option<f64> { Some(0.0) }

    fn sup(&self) -> Option<f64> { None }

    fn contains(&self, val: f64) -> bool { val >= 0.0 }
}

impl_union_intersect!(NonNegativeReals, NonNegativeReals);

impl Surjection<f64, f64> for NonNegativeReals {
    fn map_onto(&self, val: f64) -> f64 { val.max(0.0) }
}

impl fmt::Display for NonNegativeReals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{211d}(>0)")
    }
}

/// Type representing the set of strictly positive real numbers, R(>0).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PositiveReals;

impl Space for PositiveReals {
    type Value = f64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for PositiveReals {
    fn inf(&self) -> Option<f64> { Some(1e-5) }

    fn sup(&self) -> Option<f64> { None }

    fn contains(&self, val: f64) -> bool { val > 0.0 }
}

impl_union_intersect!(PositiveReals, PositiveReals);

impl Surjection<f64, f64> for PositiveReals {
    fn map_onto(&self, val: f64) -> f64 { val.max(1e-7) }
}

impl fmt::Display for PositiveReals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{211d}(≥0)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    extern crate serde_test;
    #[cfg(feature = "serialize")]
    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn test_bounded() {
        let d = Reals;

        assert_eq!(d.bounded(0.0, 1.0), Interval::bounded(0.0, 1.0));
    }

    #[test]
    fn test_card() {
        let d = Reals;

        assert_eq!(d.card(), Card::Infinite);
    }

    #[test]
    fn test_surjection() {
        let d = Reals;

        for i in -10..10 {
            let v = i as f64;

            assert_eq!(d.map_onto(v), v);
        }
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Reals;

        assert_tokens(&d, &[Token::UnitStruct { name: "Reals" }]);
    }
}
