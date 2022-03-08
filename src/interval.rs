use crate::prelude::*;
use num_traits::{Num, NumCast, ToPrimitive};
use std::{cmp, fmt, f64::{INFINITY, NEG_INFINITY}};

#[derive(Debug)]
pub enum IntervalError<V> {
    BoundsMismatch(V, V),
}

impl<V: std::fmt::Debug + std::fmt::Display> std::fmt::Display for IntervalError<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalError::BoundsMismatch(lb, ub) => write!(
                f, "Lower bound ({}) must be strictly less than the upper bound ({})", lb, ub
            ),
        }
    }
}

impl<V: std::fmt::Debug + std::fmt::Display> std::error::Error for IntervalError<V> {}

/// Generalisation of a interval.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Interval<V> {
    pub lb: Option<V>,
    pub ub: Option<V>,
}

impl<V: Num + PartialOrd> Interval<V> {
    pub fn new(lb: Option<V>, ub: Option<V>) -> Result<Interval<V>, IntervalError<V>> {
        if lb.is_some() && ub.is_some() {
            let lb = lb.unwrap();
            let ub = ub.unwrap();

            if ub <= lb {
                Err(IntervalError::BoundsMismatch(lb, ub))
            } else {
                Ok(Interval { lb: Some(lb), ub: Some(ub), })
            }
        } else {
            Ok(Interval { lb, ub, })
        }
    }

    pub fn bounded(lb: V, ub: V) -> Result<Interval<V>, IntervalError<V>> {
        Interval::new(Some(lb), Some(ub))
    }

    pub fn unit() -> Interval<V> {
        Interval { lb: Some(V::zero()), ub: Some(V::one()), }
    }

    pub fn lower_bounded(lb: V) -> Interval<V> {
        Interval { lb: Some(lb), ub: None, }
    }

    pub fn upper_bounded(ub: V) -> Interval<V> {
        Interval { lb: None, ub: Some(ub), }
    }

    pub fn unbounded() -> Interval<V> {
        Interval { lb: None, ub: None, }
    }
}

impl<V: Num + ToPrimitive + Clone + PartialOrd> Space for Interval<V> {
    const DIM: usize = 1;

    type Value = V;

    fn card(&self) -> Card {
        let test: Result<V, V::FromStrRadixErr> = Num::from_str_radix("0.1", 10);
        let is_float = test.is_ok();

        if is_float {
            Card::Infinite
        } else {
            let test: Result<V, V::FromStrRadixErr> = Num::from_str_radix("-1", 10);
            let is_signed = test.is_ok();

            if is_signed {
                match (self.lb.clone(), self.ub.clone()) {
                    (None, _) | (_, None) => Card::Infinite,
                    (Some(lb), Some(ub)) =>
                        Card::Finite(NumCast::from(ub - lb + V::one()).unwrap()),
                }
            } else {
                match (self.lb.clone(), self.ub.clone()) {
                    (_, None) => Card::Infinite,
                    (None, Some(ub)) =>
                        Card::Finite(NumCast::from(ub + V::one()).unwrap()),
                    (Some(lb), Some(ub)) =>
                        Card::Finite(NumCast::from(ub - lb + V::one()).unwrap()),
                }
            }
        }
    }

    fn contains(&self, val: &V) -> bool {
        self.lb.clone().map_or(true, |l| *val >= l) && self.ub.clone().map_or(true, |u| *val <= u)
    }
}

impl<V: Num + ToPrimitive + Clone + PartialOrd> OrderedSpace for Interval<V> {
    fn min(&self) -> Option<V> { self.lb.clone() }

    fn max(&self) -> Option<V> { self.ub.clone() }
}

impl<V: PartialEq> cmp::PartialEq for Interval<V> {
    fn eq(&self, other: &Interval<V>) -> bool { self.lb.eq(&other.lb) && self.ub.eq(&other.ub) }
}

impl<V: fmt::Display> fmt::Display for Interval<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.lb, &self.ub) {
            (Some(lb), Some(ub)) => write!(f, "[{}, {}]", lb, ub),
            (Some(lb), None) => write!(f, "[{}, {}]", lb, INFINITY),
            (None, Some(ub)) => write!(f, "[{}, {}]", NEG_INFINITY, ub),
            (None, None) => write!(f, "[{}, {}]", NEG_INFINITY, INFINITY),
        }
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
    fn test_mismatch() {
        assert!(Interval::bounded(5.0f64, 5.0f64).is_err());
        assert!(Interval::bounded(5.0f64, -5.0f64).is_err());
        assert!(Interval::bounded(-5.0f64, -5.0f64).is_err());
    }

    #[test]
    fn test_card() {
        assert_eq!(Interval::bounded(0.0f64, 5.0f64).unwrap().card(), Card::Infinite);
        assert_eq!(Interval::bounded(-5.0f64, 5.0f64).unwrap().card(), Card::Infinite);
        assert_eq!(Interval::bounded(-5.0f64, 0.0f64).unwrap().card(), Card::Infinite);

        assert_eq!(Interval::bounded(0i64, 5i64).unwrap().card(), Card::Finite(6));
        assert_eq!(Interval::bounded(-5i64, 5i64).unwrap().card(), Card::Finite(11));
        assert_eq!(Interval::bounded(-5i64, 0i64).unwrap().card(), Card::Finite(6));

        assert_eq!(Interval::bounded(0u64, 5u64).unwrap().card(), Card::Finite(6));

        assert_eq!(Interval::<i64>::unbounded().card(), Card::Infinite);
        assert_eq!(Interval::lower_bounded(0i64).card(), Card::Infinite);
        assert_eq!(Interval::upper_bounded(0i64).card(), Card::Infinite);
    }

    #[test]
    fn test_bounds_f64() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub).unwrap();

            assert_eq!(d.inf().unwrap(), lb);
            assert_eq!(d.sup().unwrap(), ub);

            assert!(d.contains(&ub));
            assert!(d.contains(&lb));
            assert!(d.contains(&((lb + ub) / 2.0)));
        }

        check(0.0, 5.0);
        check(-5.0, 5.0);
        check(-5.0, 0.0);
    }

    #[test]
    fn test_bounds_i64() {
        fn check(lb: i64, ub: i64) {
            let d = Interval::bounded(lb, ub).unwrap();

            assert_eq!(d.inf().unwrap(), lb);
            assert_eq!(d.sup().unwrap(), ub);

            assert!(d.contains(&ub));
            assert!(d.contains(&lb));
            assert!(d.contains(&((lb + ub) / 2)));
        }

        check(0, 5);
        check(-5, 5);
        check(-5, 0);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub).unwrap();

            assert_tokens(&d, &[
                Token::Struct { name: "Interval", len: 2, },
                Token::Str("lb"),
                Token::Some,
                Token::F64(lb),
                Token::Str("ub"),
                Token::Some,
                Token::F64(ub),
                Token::StructEnd,
            ]);
        }

        check(0.0, 5.0);
        check(-5.0, 5.0);
        check(-5.0, 0.0);
    }
}
