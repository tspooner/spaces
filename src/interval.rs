use crate::prelude::*;
use num_traits::{Zero, One};
use std::{
    cmp,
    fmt,
    f64::{INFINITY, NEG_INFINITY},
};

fn both<T>(opta: Option<T>, optb: Option<T>) -> Option<(T, T)> {
    match (opta, optb) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

/// Generalisation of a interval.
#[derive(Eq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Interval<T = f64> {
    pub(crate) lb: Option<T>,
    pub(crate) ub: Option<T>,
}

impl<T> Interval<T> {
    pub fn new(lb: Option<T>, ub: Option<T>) -> Interval<T> {
        Interval {
            lb, ub,
        }
    }

    pub fn unbounded() -> Interval<T> {
        Interval::new(None, None)
    }

    pub fn bounded(lb: T, ub: T) -> Interval<T> {
        Interval::new(Some(lb), Some(ub))
    }

    pub fn lower_bounded(lb: T) -> Interval<T> {
        Interval::new(Some(lb), None)
    }

    pub fn upper_bounded(ub: T) -> Interval<T> {
        Interval::new(None, Some(ub))
    }

    pub fn unit() -> Interval<T> where T: Zero + One {
        Interval::bounded(T::zero(), T::one())
    }
}

impl Space for Interval<f64> {
    type Value = f64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Infinite }

    fn contains(&self, val: &f64) -> bool {
        self.lb.map_or(true, |l| *val >= l) && self.ub.map_or(true, |u| *val <= u)
    }

    fn min(&self) -> Option<f64> { self.lb }

    fn max(&self) -> Option<f64> { self.ub }
}

impl Project<f64, f64> for Interval<f64> {
    fn project(&self, val: f64) -> f64 {
        let val = self.lb.map_or(val, |inf| val.max(inf));
        let val = self.ub.map_or(val, |sup| val.min(sup));

        val
    }
}

impl Space for Interval<i64> {
    type Value = i64;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card {
        match (self.lb, self.ub) {
            (Some(lb), Some(ub)) => Card::Finite((ub - lb + 1) as usize),
            _ => Card::Infinite,
        }
    }

    fn contains(&self, val: &i64) -> bool {
        self.lb.map_or(true, |l| *val >= l) && self.ub.map_or(true, |u| *val <= u)
    }

    fn min(&self) -> Option<i64> { self.lb }

    fn max(&self) -> Option<i64> { self.ub }
}

impl Project<i64, i64> for Interval<i64> {
    fn project(&self, val: i64) -> i64 {
        let val = self.lb.map_or(val, |inf| val.max(inf));
        let val = self.ub.map_or(val, |sup| val.min(sup));

        val
    }
}

impl<T: Clone + cmp::PartialOrd> Union for Interval<T> {
    fn union(self, other: &Self) -> Self {
        Interval::new(
            both(self.lb, other.lb.clone()).map(|(a, b)| {
                if a < b { a } else { b }
            }),
            both(self.ub, other.ub.clone()).map(|(a, b)| {
                if a > b { a } else { b }
            }),
        )
    }
}

impl<T: Clone + cmp::PartialOrd> Intersection for Interval<T> {
    fn intersect(self, other: &Self) -> Self {
        Interval::new(
            both(self.lb, other.lb.clone()).map(|(a, b)| {
                if a > b { a } else { b }
            }),
            both(self.ub, other.ub.clone()).map(|(a, b)| {
                if a < b { a } else { b }
            }),
        )
    }
}

impl<T: cmp::PartialEq> cmp::PartialEq for Interval<T> {
    fn eq(&self, other: &Interval<T>) -> bool { self.lb.eq(&other.lb) && self.ub.eq(&other.ub) }
}

impl<T: fmt::Debug> fmt::Debug for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Interval")
            .field("lb", &self.lb)
            .field("ub", &self.ub)
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for Interval<T> {
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
    fn test_card() {
        assert_eq!(Interval::bounded(0.0f64, 5.0f64).card(), Card::Infinite);
        assert_eq!(Interval::bounded(-5.0f64, 5.0f64).card(), Card::Infinite);
        assert_eq!(Interval::bounded(-5.0f64, 0.0f64).card(), Card::Infinite);

        assert_eq!(Interval::bounded(0i64, 5i64).card(), Card::Finite(6));
        assert_eq!(Interval::bounded(-5i64, 5i64).card(), Card::Finite(11));
        assert_eq!(Interval::bounded(-5i64, 0i64).card(), Card::Finite(6));

        assert_eq!(Interval::<i64>::unbounded().card(), Card::Infinite);
        assert_eq!(Interval::lower_bounded(0i64).card(), Card::Infinite);
        assert_eq!(Interval::upper_bounded(0i64).card(), Card::Infinite);
    }

    #[test]
    fn test_bounds_f64() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub);

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
            let d = Interval::bounded(lb, ub);

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

    #[test]
    fn test_surjection_f64() {
        let d = Interval::<f64>::bounded(0.0, 5.0);

        assert_eq!(d.project(-5.0), 0.0);
        assert_eq!(d.project(0.0), 0.0);
        assert_eq!(d.project(2.5), 2.5);
        assert_eq!(d.project(5.0), 5.0);
        assert_eq!(d.project(10.0), 5.0);
    }

    #[test]
    fn test_surjection_i64() {
        let d = Interval::<i64>::bounded(0, 5);

        assert_eq!(d.project(-5), 0);
        assert_eq!(d.project(0), 0);
        assert_eq!(d.project(2), 2);
        assert_eq!(d.project(5), 5);
        assert_eq!(d.project(10), 5);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub);

            assert_tokens(
                &d,
                &[
                    Token::Struct {
                        name: "Interval",
                        len: 2,
                    },
                    Token::Str("lb"),
                    Token::Some,
                    Token::F64(lb),
                    Token::Str("ub"),
                    Token::Some,
                    Token::F64(ub),
                    Token::StructEnd,
                ],
            );
        }

        check(0.0, 5.0);
        check(-5.0, 5.0);
        check(-5.0, 0.0);
    }
}
