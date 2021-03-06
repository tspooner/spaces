use crate::{Interval, Equipartition, prelude::*};
use std::fmt::{self, Display};

/// 2-dimensional heterogeneous space.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct PairSpace<D1, D2>(pub D1, pub D2);

impl<D1, D2> PairSpace<D1, D2> {
    pub fn new(d1: D1, d2: D2) -> Self { PairSpace(d1, d2) }
}

impl PairSpace<Interval, Interval> {
    pub fn equipartitioned(self, density: usize) -> PairSpace<Equipartition, Equipartition> {
        PairSpace(
            Equipartition::from_interval(self.0, density),
            Equipartition::from_interval(self.1, density),
        )
    }
}

impl<D1, D2> From<(D1, D2)> for PairSpace<D1, D2> {
    fn from(pair: (D1, D2)) -> PairSpace<D1, D2> {
        PairSpace::new(pair.0, pair.1)
    }
}

impl<D1: Space, D2: Space> Space for PairSpace<D1, D2> {
    type Value = (D1::Value, D2::Value);

    fn dim(&self) -> Dim { self.0.dim() + self.1.dim() }

    fn card(&self) -> Card { self.0.card() * self.1.card() }
}

impl<D1: Union, D2: Union> Union for PairSpace<D1, D2> {
    fn union(self, other: &Self) -> Self {
        (self.0.union(&other.0), self.1.union(&other.1)).into()
    }
}

impl<D1: Intersection, D2: Intersection> Intersection for PairSpace<D1, D2> {
    fn intersect(self, other: &Self) -> Self {
        (self.0.intersect(&other.0), self.1.intersect(&other.1)).into()
    }
}

impl<D1, X1, D2, X2> Surjection<(X1, X2), (D1::Value, D2::Value)> for PairSpace<D1, D2>
where
    D1: Space + Surjection<X1, <D1 as Space>::Value>,
    D2: Space + Surjection<X2, <D2 as Space>::Value>,
{
    fn map_onto(&self, val: (X1, X2)) -> (D1::Value, D2::Value) {
        (self.0.map_onto(val.0), self.1.map_onto(val.1))
    }
}

impl<D1: Space + Display, D2: Space + Display> fmt::Display for PairSpace<D1, D2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::discrete::Ordinal;
    use super::*;

    #[test]
    fn test_dim() {
        assert_eq!(PairSpace::new(Ordinal::new(2), Ordinal::new(2)).dim(), Dim::Finite(2));
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
        let ps = ps.equipartitioned(5);

        assert_eq!(ps.0, Equipartition::new(0.0, 5.0, 5));
        assert_eq!(ps.1, Equipartition::new(1.0, 2.0, 5));
    }

    #[test]
    fn test_surjection() {
        let ps = PairSpace::new(Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0));

        assert_eq!(ps.map_onto((6.0, 0.0)), (5.0, 1.0));
        assert_eq!(ps.map_onto((2.5, 1.5)), (2.5, 1.5));
        assert_eq!(ps.map_onto((-1.0, 10.0)), (0.0, 2.0));
    }
}
