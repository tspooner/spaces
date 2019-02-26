use continuous::Interval;
use core::*;
use std::{cmp, fmt, ops::Range};

/// Type representing a finite, uniformly partitioned interval.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Partition {
    lb: f64,
    ub: f64,
    density: usize,
}

impl Partition {
    pub fn new(lb: f64, ub: f64, density: usize) -> Partition {
        if density == 0 {
            panic!("A partition must have a density of 1 or greater.")
        }

        Partition {
            lb: lb,
            ub: ub,
            density: density,
        }
    }

    pub fn from_interval<I: Into<Interval>>(d: I, density: usize) -> Partition {
        let interval = d.into();

        Partition {
            lb: interval.lb.expect("Must be a bounded interval."),
            ub: interval.ub.expect("Must be a bounded interval."),
            density: density,
        }
    }

    #[inline]
    pub fn density(&self) -> usize { self.density }

    #[inline]
    pub fn partition_width(&self) -> f64 { (self.ub - self.lb) / self.density as f64 }

    pub fn centres(&self) -> Vec<f64> {
        let w = self.partition_width();
        let hw = w / 2.0;

        (0..self.density)
            .map(|i| self.lb + w * ((i + 1) as f64) - hw)
            .collect()
    }

    pub fn to_partition(&self, val: f64) -> usize {
        let clipped = clip!(self.lb, val, self.ub);

        let diff = clipped - self.lb;
        let range = self.ub - self.lb;

        let i = ((self.density as f64) * diff / range).floor() as usize;

        if i == self.density {
            i - 1
        } else {
            i
        }
    }
}

impl Space for Partition {
    type Value = usize;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Finite(self.density) }
}

impl BoundedSpace for Partition {
    type BoundValue = f64;

    fn inf(&self) -> Option<f64> { Some(self.lb) }

    fn sup(&self) -> Option<f64> { Some(self.ub) }

    fn contains(&self, val: Self::BoundValue) -> bool { (val >= self.lb) && (val < self.ub) }
}

impl FiniteSpace for Partition {
    fn range(&self) -> Range<Self::Value> { 0..self.density }
}

impl Enclose for Partition {
    fn enclose(self, other: &Partition) -> Partition {
        Partition::new(
            self.lb.max(other.lb),
            self.ub.max(other.ub),
            self.density.max(other.density),
        )
    }
}

impl Surjection<f64, usize> for Partition {
    fn map(&self, val: f64) -> usize { self.to_partition(val) }
}

impl Surjection<usize, usize> for Partition {
    fn map(&self, val: usize) -> usize { clip!(0, val, self.density - 1) }
}

impl cmp::PartialEq for Partition {
    fn eq(&self, other: &Partition) -> bool {
        self.lb.eq(&other.lb) && self.ub.eq(&other.ub) && self.density.eq(&other.density)
    }
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Partition")
            .field("lb", &self.lb)
            .field("ub", &self.ub)
            .field("density", &self.density)
            .finish()
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.density {
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
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_from_interval() {
        assert_eq!(
            Partition::new(0.0, 5.0, 5),
            Partition::from_interval(Interval::bounded(0.0, 5.0), 5)
        );
    }

    #[test]
    fn test_density() {
        assert_eq!(Partition::new(0.0, 5.0, 5).density(), 5);
        assert_eq!(Partition::new(0.0, 5.0, 10).density(), 10);
        assert_eq!(Partition::new(-5.0, 5.0, 100).density(), 100);
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
        assert_eq!(Partition::new(0.0, 5.0, 5).dim(), 1);
        assert_eq!(Partition::new(0.0, 5.0, 10).dim(), 1);
        assert_eq!(Partition::new(-5.0, 5.0, 10).dim(), 1);
    }

    #[test]
    fn test_card() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

            assert_eq!(d.card(), Card::Finite(density));
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_bounds() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

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
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

            assert_eq!(d.range(), 0..density);
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

    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

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
                    Token::Str("density"),
                    Token::U64(density as u64),
                    Token::StructEnd,
                ],
            );
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }
}
