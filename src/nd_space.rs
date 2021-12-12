use crate::{Equipartition, Interval, prelude::*};
use std::{
    array::{IntoIter as ArrayIntoIter},
    fmt::{self, Display},
    ops::Index,
    slice::{Iter as SliceIter},
};

/// N-dimensional space with an homogeneous basis.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct NDSpace<D, const N: usize>([D; N]);

impl<D, const N: usize> NDSpace<D, N> {
    pub fn new(dimensions: [D; N]) -> Self { NDSpace(dimensions) }

    pub fn map<F, U>(self, f: F) -> NDSpace<U, N>
        where F: FnMut(D) -> U
    {
        NDSpace(self.0.map(f))
    }

    pub fn iter(&self) -> SliceIter<D> { self.0.iter() }

    pub fn into_iter(self) -> ArrayIntoIter<D, N> { ArrayIntoIter::new(self.0) }
}

impl<D> NDSpace<D, 0> {
    pub fn empty() -> Self { NDSpace([]) }
}

impl<const N: usize> NDSpace<Interval, N> {
    pub fn equipartitioned<const M: usize>(self) -> NDSpace<Equipartition<M>, N> {
        self.map(|interval| Equipartition::from_interval(interval))
    }
}

impl<const N: usize, const M: usize> NDSpace<Equipartition<M>, N> {
    pub fn centres(&self) -> [[f64; M]; N] {
        self.0.map(|eq| eq.centres())
    }

    pub fn edges(&self) -> [[f64; M]; N] {
        self.0.map(|eq| eq.edges())
    }
}

impl<D: Space, const N: usize> Space for NDSpace<D, N> {
    type Value = [D::Value; N];

    fn dim(&self) -> Dim { self.iter().fold(Dim::Finite(0), |acc, d| acc + d.dim()) }

    fn card(&self) -> Card { self.iter().fold(Card::Finite(0), |acc, d| acc * d.card()) }

    fn contains(&self, val: &Self::Value) -> bool {
        self.0.iter()
            .zip(val.iter())
            .all(|(d, x)| d.contains(x))
    }
}

impl<D: Space + Union + Clone, const N: usize> Union for NDSpace<D, N> {
    fn union(self, other: &Self) -> Self {
        let mut i = 0;

        self.map(|d| {
            i += 1;

            d.union(&other.0[i-1])
        })
    }
}

impl<D: Space + Intersection + Clone, const N: usize> Intersection for NDSpace<D, N> {
    fn intersect(self, other: &Self) -> Self {
        let mut i = 0;

        self.map(|d| {
            i += 1;

            d.intersect(&other.0[i-1])
        })
    }
}

impl<D, X, const N: usize> Projection<[X; N], [D::Value; N]> for NDSpace<D, N>
where
    D: Space + Projection<X, <D as Space>::Value>,
    D::Value: Default,
{
    fn project(&self, val: [X; N]) -> [D::Value; N] {
        let mut i = 0;

        val.map(|x| {
            i += 1;

            self.0[i-1].project(x)
        })
    }
}

impl<D: Space, const N: usize> Index<usize> for NDSpace<D, N> {
    type Output = D;

    fn index(&self, index: usize) -> &D { self.0.index(index) }
}

impl<D, const N: usize> IntoIterator for NDSpace<D, N> {
    type Item = D;
    type IntoIter = ArrayIntoIter<D, N>;

    fn into_iter(self) -> Self::IntoIter { self.into_iter() }
}

impl<D: Space + Display, const N: usize> fmt::Display for NDSpace<D, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for (i, v) in self.0.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }

            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::discrete::Ordinal;
    use std::iter::FromIterator;
    use super::*;

    #[test]
    fn test_dim() {
        assert_eq!(NDSpace::new([Ordinal::new(2); 2]).dim(), Dim::Finite(2));
    }

    #[test]
    fn test_card() {
        assert_eq!(
            NDSpace::new([Ordinal::new(2); 2]).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_union() {
        let s1 = NDSpace::new([Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 3.0)]);
        let s2 = NDSpace::new([Interval::bounded(-5.0, 0.0), Interval::bounded(1.0, 2.0)]);

        assert_eq!(s1.union(&s2), NDSpace::new([
            Interval::bounded(-5.0, 5.0),
            Interval::bounded(1.0, 3.0)
        ]));
    }

    #[test]
    fn test_surjection() {
        let space = NDSpace::new([Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)]);

        assert_eq!(space.project([6.0, 0.0]), [5.0, 1.0]);
        assert_eq!(space.project([2.5, 1.5]), [2.5, 1.5]);
        assert_eq!(space.project([-1.0, 3.0]), [0.0, 2.0]);
    }

    #[test]
    fn test_indexing() {
        let dimensions = [Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];
        let space = NDSpace::new(dimensions.clone());

        assert_eq!(space[0], dimensions[0]);
        assert_eq!(space[1], dimensions[1]);
    }

    #[test]
    fn test_iteration() {
        let dimensions = [Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];
        let space = NDSpace::new(dimensions.clone());

        assert_eq!(space.into_iter().collect::<Vec<Interval>>(), dimensions);
    }
}
