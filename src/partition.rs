use crate::{Space, BoundedSpace, FiniteSpace, Interval, core::*};
use std::{cmp, fmt, ops::Range};

/// Type representing a finite, uniformly partitioned interval.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Partition {
    lb: f64,
    ub: f64,
    n_partitions: usize,
}

impl Partition {
    pub fn new(lb: f64, ub: f64, n_partitions: usize) -> Partition {
        if n_partitions == 0 {
            panic!("A partition must have a number partitions of 1 or greater.")
        }

        Partition { lb, ub, n_partitions, }
    }

    pub fn from_interval<I: Into<Interval>>(d: I, n_partitions: usize) -> Partition {
        let interval = d.into();

        Partition {
            lb: interval.lb.expect("Must be a bounded interval."),
            ub: interval.ub.expect("Must be a bounded interval."),
            n_partitions,
        }
    }

    #[inline]
    pub fn n_partitions(&self) -> usize { self.n_partitions }

    #[inline]
    pub fn partition_width(&self) -> f64 { (self.ub - self.lb) / self.n_partitions as f64 }

    pub fn centres(&self) -> Vec<f64> {
        let w = self.partition_width();
        let hw = w / 2.0;

        (0..self.n_partitions)
            .map(|i| self.lb + w * ((i + 1) as f64) - hw)
            .collect()
    }

    pub fn edges(&self) -> Vec<f64> {
        let w = self.partition_width();

        (0..=self.n_partitions)
            .map(|i| self.lb + w * (i as f64))
            .collect()
    }

    pub fn to_partition(&self, val: f64) -> usize {
        let clipped = clip!(self.lb, val, self.ub);

        let diff = clipped - self.lb;
        let range = self.ub - self.lb;

        let i = ((self.n_partitions as f64) * diff / range).floor() as usize;

        if i == self.n_partitions {
            i - 1
        } else {
            i
        }
    }
}

impl Space for Partition {
    type Value = usize;

    fn dim(&self) -> Dim { Dim::one() }

    fn card(&self) -> Card { Card::Finite(self.n_partitions) }
}

impl BoundedSpace for Partition {
    fn inf(&self) -> Option<usize> { Some(0) }

    fn sup(&self) -> Option<usize> { Some(self.n_partitions) }

    fn contains(&self, val: usize) -> bool { val < self.n_partitions }
}

impl FiniteSpace for Partition {
    fn range(&self) -> Range<Self::Value> { 0..self.n_partitions }
}

impl Surjection<f64, usize> for Partition {
    fn map(&self, val: f64) -> usize { self.to_partition(val) }
}

impl Surjection<usize, usize> for Partition {
    fn map(&self, val: usize) -> usize { clip!(0, val, self.n_partitions - 1) }
}

impl cmp::PartialEq for Partition {
    fn eq(&self, other: &Partition) -> bool {
        self.lb.eq(&other.lb) && self.ub.eq(&other.ub) && self.n_partitions.eq(&other.n_partitions)
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.n_partitions {
            d if d == 1 =>
                write!(f, "{{{} = x0, x1 = {}}}", self.lb, self.ub),
            d if d == 2 =>
                write!(f, "{{{} = x0, x1, x2 = {}}}", self.lb, self.ub),
            d =>
                write!(f, "{{{} = x0, x1, ..., x{} = {}}}", self.lb, d, self.ub),
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
            Partition::new(0.0, 5.0, 5),
            Partition::from_interval(Interval::bounded(0.0, 5.0), 5)
        );
    }

    #[test]
    fn test_density() {
        assert_eq!(Partition::new(0.0, 5.0, 5).n_partitions(), 5);
        assert_eq!(Partition::new(0.0, 5.0, 10).n_partitions(), 10);
        assert_eq!(Partition::new(-5.0, 5.0, 100).n_partitions(), 100);
    }

    #[test]
    fn test_partition_width() {
        assert_eq!(Partition::new(0.0, 5.0, 5).partition_width(), 1.0);
        assert_eq!(Partition::new(0.0, 5.0, 10).partition_width(), 0.5);
        assert_eq!(Partition::new(-5.0, 5.0, 10).partition_width(), 1.0);
    }

    #[test]
    fn test_centres() {
        assert_eq!(
            Partition::new(0.0, 5.0, 5).centres(),
            vec![0.5, 1.5, 2.5, 3.5, 4.5]
        );

        assert_eq!(
            Partition::new(-5.0, 5.0, 5).centres(),
            vec![-4.0, -2.0, 0.0, 2.0, 4.0]
        );
    }

    #[test]
    fn test_to_partition() {
        let d = Partition::new(0.0, 5.0, 6);

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
        assert_eq!(Partition::new(0.0, 5.0, 5).dim(), Dim::one());
        assert_eq!(Partition::new(0.0, 5.0, 10).dim(), Dim::one());
        assert_eq!(Partition::new(-5.0, 5.0, 10).dim(), Dim::one());
    }

    #[test]
    fn test_card() {
        fn check(lb: f64, ub: f64, n_partitions: usize) {
            let d = Partition::new(lb, ub, n_partitions);

            assert_eq!(d.card(), Card::Finite(n_partitions));
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_bounds() {
        fn check(lb: f64, ub: f64, n_partitions: usize) {
            let d = Partition::new(lb, ub, n_partitions);

            assert_eq!(d.inf().unwrap(), lb);
            assert_eq!(d.sup().unwrap(), ub);

            assert!(!d.contains(ub));
            assert!(d.contains(lb));
            assert!(d.contains((lb + ub) / 2.0));
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_range() {
        fn check(lb: f64, ub: f64, n_partitions: usize) {
            let d = Partition::new(lb, ub, n_partitions);

            assert_eq!(d.range(), 0..n_partitions);
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_surjection_f64() {
        let d = Partition::new(0.0, 5.0, 6);

        assert_eq!(d.map(-1.0), 0);
        assert_eq!(d.map(0.0), 0);
        assert_eq!(d.map(1.0), 1);
        assert_eq!(d.map(2.0), 2);
        assert_eq!(d.map(3.0), 3);
        assert_eq!(d.map(4.0), 4);
        assert_eq!(d.map(5.0), 5);
        assert_eq!(d.map(6.0), 5);
    }

    #[test]
    fn test_surjection_usize() {
        let d = Partition::new(5.0, 6.0, 2);

        assert_eq!(d.map(0), 0);
        assert_eq!(d.map(1), 1);
        assert_eq!(d.map(2), 1);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64, n_partitions: usize) {
            let d = Partition::new(lb, ub, n_partitions);

            assert_tokens(
                &d,
                &[
                    Token::Struct {
                        name: "Partition",
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
