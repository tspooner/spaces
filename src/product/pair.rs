use continuous::Interval;
use core::{Space, Card, Surjection};
use discrete::Partition;
use std::fmt::{self, Display};

/// 2-dimensional homogeneous space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PairSpace<D1, D2>(pub D1, pub D2)
where
    D1: Space,
    D2: Space;

impl<D1: Space, D2: Space> PairSpace<D1, D2> {
    pub fn new(d1: D1, d2: D2) -> Self { PairSpace(d1, d2) }
}

impl PairSpace<Interval, Interval> {
    pub fn partitioned(self, density: usize) -> PairSpace<Partition, Partition> {
        PairSpace(
            Partition::from_interval(self.0, density),
            Partition::from_interval(self.1, density),
        )
    }
}

impl<D1: Space, D2: Space> Space for PairSpace<D1, D2> {
    type Value = (D1::Value, D2::Value);

    fn dim(&self) -> usize { 2 }

    fn card(&self) -> Card { self.0.card() * self.1.card() }
}

impl<D1, X1, D2, X2> Surjection<(X1, X2), (D1::Value, D2::Value)> for PairSpace<D1, D2>
where
    D1: Space + Surjection<X1, <D1 as Space>::Value>,
    D2: Space + Surjection<X2, <D2 as Space>::Value>,
{
    fn map(&self, val: (X1, X2)) -> (D1::Value, D2::Value) {
        (self.0.map(val.0), self.1.map(val.1))
    }
}

impl<D1: Space + Display, D2: Space + Display> fmt::Display for PairSpace<D1, D2> {
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
    use product::PairSpace;

    #[test]
    fn test_dim() {
        assert_eq!(PairSpace::new(Ordinal::new(2), Ordinal::new(2)).dim(), 2);
    }

    #[test]
    fn test_card() {
        assert_eq!(
            PairSpace::new(Ordinal::new(2), Ordinal::new(2)).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_partitioned() {
        let ps = PairSpace::new(Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0));
        let ps = ps.partitioned(5);

        assert_eq!(ps.0, Partition::new(0.0, 5.0, 5));
        assert_eq!(ps.1, Partition::new(1.0, 2.0, 5));
    }

    #[test]
    fn test_surjection() {
        let ps = PairSpace::new(Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0));

        assert_eq!(ps.map((6.0, 0.0)), (5.0, 1.0));
        assert_eq!(ps.map((2.5, 1.5)), (2.5, 1.5));
        assert_eq!(ps.map((-1.0, 10.0)), (0.0, 2.0));
    }
}
