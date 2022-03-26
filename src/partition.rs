use crate::{Interval, prelude::*};
use num_traits::{Float, NumCast};
use std::{cmp, fmt, ops::Range};

#[derive(Debug)]
pub enum PartitionError<V> {
    BoundsMismatch(V, V),
    EmptyPartitioning,
    UnboundedInterval(Interval<V>)
}

impl<V: std::fmt::Debug + std::fmt::Display> std::fmt::Display for PartitionError<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartitionError::BoundsMismatch(lb, ub) => write!(
                f, "Lower bound ({}) must be strictly less than the upper bound ({})", lb, ub
            ),
            PartitionError::EmptyPartitioning => write!(
                f, "Partition number must be non-zero."
            ),
            PartitionError::UnboundedInterval(interval) => write!(
                f, "Underlying interval ({}) must be fully bounded.", interval
            ),
        }
    }
}

/// Finite, uniformly partitioned interval.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Equipartition<const N: usize, V> {
    pub lb: V,
    pub ub: V,
}

impl<const N: usize, V: Float> Equipartition<N, V> {
    pub fn new(lb: V, ub: V) -> Result<Equipartition<N, V>, PartitionError<V>> {
        if N == 0 {
            Err(PartitionError::EmptyPartitioning)
        } else if ub <= lb {
            Err(PartitionError::BoundsMismatch(lb, ub))
        } else {
            Ok(Equipartition { lb, ub, })
        }
    }

    pub fn from_interval<I: Into<Interval<V>>>(d: I) -> Result<Equipartition<N, V>, PartitionError<V>> {
        let interval: Interval<V> = d.into();

        if let (Some(l), Some(u)) = (interval.lb, interval.ub) {
            Ok(Equipartition { lb: l, ub: u, })
        } else {
            Err(PartitionError::UnboundedInterval(interval))
        }
    }

    #[inline]
    pub fn n_partitions(&self) -> usize { N }

    #[inline]
    pub fn partition_width(&self) -> V { (self.ub - self.lb) / NumCast::from(N).unwrap() }

    pub fn centres(&self) -> [V; N] {
        let w = self.partition_width();
        let hw = w / (V::one() + V::one());
        let mut output = [V::zero(); N];

        for i in 0..N {
            output[i] = self.lb + w * NumCast::from(i + 1).unwrap() - hw;
        }

        output
    }

    pub fn edges(&self) -> [V; N] {
        let w = self.partition_width();
        let mut output = [V::zero(); N];

        for i in 0..N {
            output[i] = self.lb + w * NumCast::from(i).unwrap();
        }

        output
    }

    pub fn to_partition(&self, val: V) -> usize {
        let clipped = clip!(self.lb, val, self.ub);

        let diff = clipped - self.lb;
        let range = self.ub - self.lb;

        let i = (diff * NumCast::from(N).unwrap() / range).floor();
        let i: usize = NumCast::from(i).unwrap();

        if i >= N { N - 1 } else { i }
    }
}

impl<const N: usize, V: Float> Space for Equipartition<N, V> {
    const DIM: usize = 1;

    type Value = usize;

    fn card(&self) -> Card { Card::Finite(N) }

    fn contains(&self, val: &usize) -> bool { *val < N }
}

impl<const N: usize, V: Float> OrderedSpace for Equipartition<N, V> {
    fn min(&self) -> Option<usize> { Some(0) }

    fn max(&self) -> Option<usize> { Some(N - 1) }
}

impl<const N: usize, V: Float> FiniteSpace for Equipartition<N, V> {
    fn to_ordinal(&self) -> Range<Self::Value> { 0..N }
}

impl<const N: usize, V: PartialEq, const M: usize> cmp::PartialEq<Equipartition<M, V>> for Equipartition<N, V> {
    fn eq(&self, other: &Equipartition<M, V>) -> bool {
        N.eq(&M) && self.lb.eq(&other.lb) && self.ub.eq(&other.ub)
    }
}

impl<const N: usize, V: fmt::Display> fmt::Display for Equipartition<N, V> {
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
            Equipartition::<5, f64>::new(0.0, 5.0).unwrap(),
            Equipartition::<5, f64>::from_interval(Interval::bounded(0.0, 5.0).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_density() {
        assert_eq!(Equipartition::<5, f32>::new(0.0, 5.0).unwrap().n_partitions(), 5);
        assert_eq!(Equipartition::<10, f32>::new(0.0, 5.0).unwrap().n_partitions(), 10);
        assert_eq!(Equipartition::<100, f64>::new(-5.0, 5.0).unwrap().n_partitions(), 100);
    }

    #[test]
    fn test_partition_width() {
        assert_eq!(Equipartition::<5, f32>::new(0.0, 5.0).unwrap().partition_width(), 1.0);
        assert_eq!(Equipartition::<10, f32>::new(0.0, 5.0).unwrap().partition_width(), 0.5);
        assert_eq!(Equipartition::<100, f64>::new(-5.0, 5.0).unwrap().partition_width(), 0.1);
    }

    #[test]
    fn test_centres() {
        assert_eq!(
            Equipartition::new(0.0, 5.0).unwrap().centres(),
            [0.5, 1.5, 2.5, 3.5, 4.5]
        );

        assert_eq!(
            Equipartition::new(-5.0, 5.0).unwrap().centres(),
            [-4.0, -2.0, 0.0, 2.0, 4.0]
        );
    }

    #[test]
    fn test_to_partition() {
        let d = Equipartition::<6, f64>::new(0.0, 5.0).unwrap();

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
        assert_eq!(Equipartition::<1, f32>::DIM, 1);
        assert_eq!(Equipartition::<5, f32>::DIM, 1);
        assert_eq!(Equipartition::<10, f64>::DIM, 1);
    }

    #[test]
    fn test_card() {
        fn check<const N: usize>(lb: f64, ub: f64) {
            let d = Equipartition::<N, f64>::new(lb, ub).unwrap();

            assert_eq!(d.card(), Card::Finite(N));
        }

        check::<5>(0.0, 5.0);
        check::<5>(-5.0, 0.0);
        check::<10>(-5.0, 5.0);
    }

    #[test]
    fn test_to_ordinal() {
        fn check<const N: usize>(lb: f64, ub: f64) {
            let d = Equipartition::<N, f64>::new(lb, ub).unwrap();

            assert_eq!(d.to_ordinal(), 0..N);
        }

        check::<5>(0.0, 5.0);
        check::<5>(-5.0, 0.0);
        check::<10>(-5.0, 5.0);
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        fn check<const N: usize>(lb: f64, ub: f64) {
            let d = Equipartition::<N, f64>::new(lb, ub).unwrap();

            assert_tokens(&d, &[
                Token::Struct { name: "Equipartition", len: 2, },
                Token::Str("lb"),
                Token::F64(lb),
                Token::Str("ub"),
                Token::F64(ub),
                Token::StructEnd,
            ]);
        }

        check::<5>(0.0, 5.0);
        check::<10>(-5.0, 5.0);
        check::<5>(-5.0, 0.0);
    }
}
