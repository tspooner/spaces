use crate::{Space, Dim, Card, Union, Surjection, Interval, Partition};
use std::{
    fmt::{self, Display},
    ops::Index,
    slice::{Iter as SliceIter},
};

/// 2-dimensional homogeneous space.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct TwoSpace<D>([D; 2]);

impl<D> TwoSpace<D> {
    pub fn new(ds: [D; 2]) -> Self { TwoSpace(ds) }

    pub fn iter(&self) -> SliceIter<D> { self.0.iter() }
}

impl TwoSpace<Interval> {
    pub fn partitioned(self, density: usize) -> TwoSpace<Partition> {
        TwoSpace([
            Partition::from_interval(self[0], density),
            Partition::from_interval(self[1], density),
        ])
    }
}

impl<D: Space> Space for TwoSpace<D> {
    type Value = [D::Value; 2];

    fn dim(&self) -> Dim { self[0].dim() + self[1].dim() }

    fn card(&self) -> Card { self[0].card() * self[1].card() }
}

impl<D: Union + Clone> Union for TwoSpace<D> {
    fn union(self, other: &Self) -> Self {
        let TwoSpace([d1, d2]) = self;

        [d1.union(&other[0]), d2.union(&other[1])].into()
    }
}

impl<D, X> Surjection<[X; 2], [D::Value; 2]> for TwoSpace<D>
where
    D: Space + Surjection<X, <D as Space>::Value>,
    X: Clone,
{
    fn map(&self, val: [X; 2]) -> [D::Value; 2] {
        [self[0].map(val[0].clone()), self[1].map(val[1].clone())]
    }
}

impl<D> Index<usize> for TwoSpace<D> {
    type Output = D;

    fn index(&self, idx: usize) -> &D { self.0.index(idx) }
}

impl<D: Clone> From<[D; 2]> for TwoSpace<D> {
    fn from(pair: [D; 2]) -> TwoSpace<D> {
        TwoSpace::new(pair.clone())
    }
}

impl<D: Space + Display> fmt::Display for TwoSpace<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self[0], self[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::discrete::Ordinal;
    use super::*;

    #[test]
    fn test_dim() {
        assert_eq!(TwoSpace::new([Ordinal::new(2), Ordinal::new(2)]).dim(), Dim::Finite(2));
    }

    #[test]
    fn test_card() {
        assert_eq!(
            TwoSpace::new([Ordinal::new(2), Ordinal::new(2)]).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_partitioned() {
        let ps = TwoSpace::new([
            Interval::bounded(0.0, 5.0),
            Interval::bounded(1.0, 2.0)
        ]).partitioned(5);

        assert_eq!(ps[0], Partition::new(0.0, 5.0, 5));
        assert_eq!(ps[1], Partition::new(1.0, 2.0, 5));
    }

    #[test]
    fn test_surjection() {
        let ps = TwoSpace::new([
            Interval::bounded(0.0, 5.0),
            Interval::bounded(1.0, 2.0)
        ]);

        assert_eq!(ps.map([6.0, 0.0]), [5.0, 1.0]);
        assert_eq!(ps.map([2.5, 1.5]), [2.5, 1.5]);
        assert_eq!(ps.map([-1.0, 10.0]), [0.0, 2.0]);
    }
}
