use continuous::Interval;
use core::*;
use discrete::Partition;
use std::fmt::{self, Display};

/// 2-dimensional homogeneous space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct DoubleSpace<D>(pub D, pub D);

impl<D> DoubleSpace<D> {
    pub fn new(d1: D, d2: D) -> Self { DoubleSpace(d1, d2) }
}

impl DoubleSpace<Interval> {
    pub fn partitioned(self, density: usize) -> DoubleSpace<Partition> {
        DoubleSpace(
            Partition::from_interval(self.0, density),
            Partition::from_interval(self.1, density),
        )
    }
}

impl<D: Clone> From<[D; 2]> for DoubleSpace<D> {
    fn from(pair: [D; 2]) -> DoubleSpace<D> {
        DoubleSpace::new(pair[0].clone(), pair[1].clone())
    }
}

impl<D: Space> Space for DoubleSpace<D> {
    type Value = [D::Value; 2];

    fn dim(&self) -> usize { 2 }

    fn card(&self) -> Card { self.0.card() * self.1.card() }
}

impl<D: Enclose + Clone> Enclose for DoubleSpace<D> {
    fn enclose(self, other: &Self) -> Self {
        [self.0.enclose(&other.0), self.1.enclose(&other.1)].into()
    }
}

impl<D, X> Surjection<[X; 2], [D::Value; 2]> for DoubleSpace<D>
where
    D: Space + Surjection<X, <D as Space>::Value>,
    X: Clone,
{
    fn map(&self, val: [X; 2]) -> [D::Value; 2] {
        [self.0.map(val[0].clone()), self.1.map(val[1].clone())]
    }
}

impl<D: Space + Display> fmt::Display for DoubleSpace<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    extern crate ndarray;

    use core::{Space, Card, Surjection};
    use continuous::Interval;
    use discrete::{Ordinal, Partition};
    use product::DoubleSpace;

    #[test]
    fn test_dim() {
        assert_eq!(DoubleSpace::new(Ordinal::new(2), Ordinal::new(2)).dim(), 2);
    }

    #[test]
    fn test_card() {
        assert_eq!(
            DoubleSpace::new(Ordinal::new(2), Ordinal::new(2)).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_partitioned() {
        let ps = DoubleSpace::new(Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0));
        let ps = ps.partitioned(5);

        assert_eq!(ps.0, Partition::new(0.0, 5.0, 5));
        assert_eq!(ps.1, Partition::new(1.0, 2.0, 5));
    }

    #[test]
    fn test_surjection() {
        let ps = DoubleSpace::new(Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0));

        assert_eq!(ps.map([6.0, 0.0]), [5.0, 1.0]);
        assert_eq!(ps.map([2.5, 1.5]), [2.5, 1.5]);
        assert_eq!(ps.map([-1.0, 10.0]), [0.0, 2.0]);
    }
}
