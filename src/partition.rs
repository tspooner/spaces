use crate::{Interval, prelude::*};
use std::{cmp, fmt, ops::Range};

/// Finite, uniformly partitioned interval.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Equipartition<const N: usize> {
    lb: f64,
    ub: f64,
}

impl<const N: usize> Equipartition<N> {
    pub fn new(lb: f64, ub: f64) -> Equipartition<N> {
        if N == 0 {
            panic!("A partition must have a number partitions of 1 or greater.")
        }

        Equipartition { lb, ub, }
    }

    pub fn from_interval<I: Into<Interval>>(d: I) -> Equipartition<N> {
        let interval = d.into();

        Equipartition {
            lb: interval.lb.expect("Must be a bounded interval."),
            ub: interval.ub.expect("Must be a bounded interval."),
        }
    }

    #[inline]
    pub fn n_partitions(&self) -> usize { N }

    #[inline]
    pub fn partition_width(&self) -> f64 { (self.ub - self.lb) / N as f64 }

    pub fn centres(&self) -> [f64; N] {
        let w = self.partition_width();
        let hw = w / 2.0;
        let mut output = [f64::default(); N];

        for i in 0..N {
            output[i] = self.lb + w * ((i + 1) as f64) - hw;
        }

        output
    }

    pub fn edges(&self) -> [f64; N] {
        let w = self.partition_width();
        let mut output = [f64::default(); N];

        for i in 0..N {
            output[i] = self.lb + w * (i as f64);
        }

        output
    }

    pub fn to_partition(&self, val: f64) -> usize {
        let clipped = clip!(self.lb, val, self.ub);

        let diff = clipped - self.lb;
        let range = self.ub - self.lb;

        let i = ((N as f64) * diff / range).floor() as usize;

        if i >= N { N - 1 } else { i }
    }
}

impl<const N: usize> Space for Equipartition<N> {
    type Value = usize;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Finite(N) }

    fn contains(&self, val: &usize) -> bool { *val < N }
}

impl<const N: usize> OrderedSpace for Equipartition<N> {
    fn min(&self) -> Option<usize> { Some(0) }

    fn max(&self) -> Option<usize> { Some(N - 1) }
}

impl<const N: usize> FiniteSpace for Equipartition<N> {
    fn to_ordinal(&self) -> Range<Self::Value> { 0..N }
}

impl<const N: usize> Project<f64, usize> for Equipartition<N> {
    fn project(&self, val: f64) -> usize { self.to_partition(val) }
}

impl<const N: usize, const M: usize> cmp::PartialEq<Equipartition<M>> for Equipartition<N> {
    fn eq(&self, other: &Equipartition<M>) -> bool {
        N.eq(&M) && self.lb.eq(&other.lb) && self.ub.eq(&other.ub)
    }
}

impl<const N: usize> fmt::Display for Equipartition<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match N {
            n if n == 1 => write!(f, "{{{} = x0, x1 = {}}}", self.lb, self.ub),
            n if n == 2 => write!(f, "{{{} = x0, x1, x2 = {}}}", self.lb, self.ub),
            n => write!(f, "{{{} = x0, x1, ..., x{} = {}}}", self.lb, n, self.ub),
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
    fn test_from_interval() {
        assert_eq!(
            Equipartition::<5>::new(0.0, 5.0),
            Equipartition::<5>::from_interval(Interval::bounded(0.0, 5.0))
        );
    }

    #[test]
    fn test_density() {
        assert_eq!(Equipartition::<5>::new(0.0, 5.0).n_partitions(), 5);
        assert_eq!(Equipartition::<10>::new(0.0, 5.0).n_partitions(), 10);
        assert_eq!(Equipartition::<100>::new(-5.0, 5.0).n_partitions(), 100);
    }

    #[test]
    fn test_partition_width() {
        assert_eq!(Equipartition::<5>::new(0.0, 5.0).partition_width(), 1.0);
        assert_eq!(Equipartition::<10>::new(0.0, 5.0).partition_width(), 0.5);
        assert_eq!(Equipartition::<10>::new(-5.0, 5.0).partition_width(), 1.0);
    }

    #[test]
    fn test_centres() {
        assert_eq!(
            Equipartition::new(0.0, 5.0).centres(),
            [0.5, 1.5, 2.5, 3.5, 4.5]
        );

        assert_eq!(
            Equipartition::new(-5.0, 5.0).centres(),
            [-4.0, -2.0, 0.0, 2.0, 4.0]
        );
    }

    #[test]
    fn test_to_partition() {
        let d = Equipartition::<6>::new(0.0, 5.0);

        assert_eq!(d.to_partition(-1.0), 0);
        assert_eq!(d.to_partition(0.0), 0);
        assert_eq!(d.to_partition(1.0), 1);
        assert_eq!(d.to_partition(2.0), 2);
        assert_eq!(d.to_partition(3.0), 3);
        assert_eq!(d.to_partition(4.0), 4);
        assert_eq!(d.to_partition(5.0), 5);
        assert_eq!(d.to_partition(6.0), 5);
    }

    #[test]
    fn test_dim() {
        assert_eq!(Equipartition::<5>::new(0.0, 5.0).dim(), Dim::one());
        assert_eq!(Equipartition::<5>::new(0.0, 5.0).dim(), Dim::one());
        assert_eq!(Equipartition::<10>::new(-5.0, 5.0).dim(), Dim::one());
    }

    #[test]
    fn test_card() {
        fn check<const N: usize>(lb: f64, ub: f64) {
            let d = Equipartition::<N>::new(lb, ub);

            assert_eq!(d.card(), Card::Finite(N));
        }

        check::<5>(0.0, 5.0);
        check::<5>(-5.0, 0.0);
        check::<10>(-5.0, 5.0);
    }

    #[test]
    fn test_bounds() {
        fn check<const N: usize>(lb: f64, ub: f64) {
            let d = Equipartition::<N>::new(lb, ub);

            assert_eq!(d.inf().unwrap(), 0);
            assert_eq!(d.sup().unwrap(), N - 1);

            assert!(d.contains(&d.project(ub)));
            assert!(d.contains(&d.project(lb)));
            assert!(d.contains(&d.project((lb + ub) / 2.0)));
        }

        check::<5>(0.0, 5.0);
        check::<5>(-5.0, 0.0);
        check::<10>(-5.0, 5.0);
    }

    #[test]
    fn test_to_ordinal() {
        fn check<const N: usize>(lb: f64, ub: f64) {
            let d = Equipartition::<N>::new(lb, ub);

            assert_eq!(d.to_ordinal(), 0..N);
        }

        check::<5>(0.0, 5.0);
        check::<5>(-5.0, 0.0);
        check::<10>(-5.0, 5.0);
    }

    #[test]
    fn test_surjection_f64() {
        let d = Equipartition::<6>::new(0.0, 5.0);

        assert_eq!(d.project(-1.0), 0);
        assert_eq!(d.project(0.0), 0);
        assert_eq!(d.project(1.0), 1);
        assert_eq!(d.project(2.0), 2);
        assert_eq!(d.project(3.0), 3);
        assert_eq!(d.project(4.0), 4);
        assert_eq!(d.project(5.0), 5);
        assert_eq!(d.project(6.0), 5);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64, n_partitions: usize) {
            let d = Equipartition::new(lb, ub, n_partitions);

            assert_tokens(
                &d,
                &[
                    Token::Struct {
                        name: "Equipartition",
                        len: 3,
                    },
                    Token::Str("lb"),
                    Token::F64(lb),
                    Token::Str("ub"),
                    Token::F64(ub),
                    Token::Str("n_partitions"),
                    Token::U64(n_partitions as u64),
                    Token::StructEnd,
                ],
            );
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }
}
