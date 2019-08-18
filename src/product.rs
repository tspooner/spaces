use crate::{
    Space, Dim, Card, Surjection, Union,
    Equipartition, Interval,
};
use itertools::{Itertools, EitherOrBoth};
use std::{
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Add, Index},
    slice::{Iter as SliceIter},
    vec::{IntoIter as VecIntoIter},
};

/// N-dimensional homogeneous space.
///
/// Represention of the Cartesian product of N topological spaces.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ProductSpace<D>(Vec<D>);

impl<D> ProductSpace<D> {
    pub fn new(dimensions: Vec<D>) -> Self { ProductSpace(dimensions) }

    pub fn empty() -> Self { ProductSpace(vec![]) }

    pub fn iter(&self) -> SliceIter<D> { self.0.iter() }

    pub fn into_iter(self) -> VecIntoIter<D> { self.0.into_iter() }
}

impl ProductSpace<Interval> {
    pub fn equipartitioned(self, density: usize) -> ProductSpace<Equipartition> {
        self.into_iter()
            .map(|d| Equipartition::from_interval(d, density))
            .collect()
    }
}

impl ProductSpace<Equipartition> {
    pub fn centres(&self) -> Vec<Vec<f64>> {
        self.iter().map(|d| d.centres()).collect()
    }

    pub fn edges(&self) -> Vec<Vec<f64>> {
        self.iter().map(|d| d.edges()).collect()
    }
}

impl<D: Space> Space for ProductSpace<D> {
    type Value = Vec<D::Value>;

    fn dim(&self) -> Dim { self.iter().fold(Dim::Finite(0), |acc, d| acc + d.dim()) }

    fn card(&self) -> Card { self.iter().fold(Card::Finite(0), |acc, d| acc * d.card()) }
}

impl<D: Space + Union + Clone> Union for ProductSpace<D> {
    fn union(self, other: &Self) -> Self {
        use self::EitherOrBoth::*;

        self.into_iter()
            .zip_longest(other.iter())
            .map(|el| match el {
                Both(l, r) => l.union(r),
                Left(l) => l,
                Right(r) => r.clone(),
            })
            .collect()
    }
}

impl<D, X> Surjection<Vec<X>, Vec<D::Value>> for ProductSpace<D>
where D: Space + Surjection<X, <D as Space>::Value>
{
    fn map_onto(&self, val: Vec<X>) -> Vec<D::Value> {
        self.iter()
            .zip(val.into_iter())
            .map(|(d, v)| d.map_onto(v))
            .collect()
    }
}

impl<D: Space> Index<usize> for ProductSpace<D> {
    type Output = D;

    fn index(&self, index: usize) -> &D { self.0.index(index) }
}

impl<D> FromIterator<D> for ProductSpace<D> {
    fn from_iter<I: IntoIterator<Item = D>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<D: Space> IntoIterator for ProductSpace<D> {
    type Item = D;
    type IntoIter = ::std::vec::IntoIter<D>;

    fn into_iter(self) -> Self::IntoIter { self.into_iter() }
}

impl<D: Space> Add<D> for ProductSpace<D> {
    type Output = Self;

    fn add(mut self, rhs: D) -> Self::Output { self.0.push(rhs); self }
}

impl<D: Space> Add<ProductSpace<D>> for ProductSpace<D> {
    type Output = Self;

    fn add(self, rhs: ProductSpace<D>) -> Self::Output {
        FromIterator::from_iter(self.into_iter().chain(rhs.into_iter()))
    }
}

impl<D: Space + Display> fmt::Display for ProductSpace<D> {
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
        assert_eq!(ProductSpace::new(vec![Ordinal::new(2); 2]).dim(), Dim::Finite(2));
    }

    #[test]
    fn test_card() {
        assert_eq!(
            ProductSpace::new(vec![Ordinal::new(2); 2]).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_union() {
        let s1 = ProductSpace::new(vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 3.0)]);
        let s2 = ProductSpace::new(vec![Interval::bounded(-5.0, 0.0), Interval::bounded(1.0, 2.0)]);

        assert_eq!(s1.union(&s2), ProductSpace::new(vec![
            Interval::bounded(-5.0, 5.0),
            Interval::bounded(1.0, 3.0)
        ]));
    }

    #[test]
    fn test_surjection() {
        let space = ProductSpace::new(vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)]);

        assert_eq!(space.map_onto(vec![6.0, 0.0]), vec![5.0, 1.0]);
        assert_eq!(space.map_onto(vec![2.5, 1.5]), vec![2.5, 1.5]);
        assert_eq!(space.map_onto(vec![-1.0, 3.0]), vec![0.0, 2.0]);
    }

    #[test]
    fn test_indexing() {
        let dimensions = vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];
        let space = ProductSpace::from_iter(dimensions.iter().cloned());

        assert_eq!(space[0], dimensions[0]);
        assert_eq!(space[1], dimensions[1]);
    }

    #[test]
    fn test_iteration() {
        let dimensions = vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];
        let space = ProductSpace::from_iter(dimensions.iter().cloned());

        assert_eq!(space.into_iter().collect::<Vec<Interval>>(), dimensions);
    }

    #[test]
    fn test_add_op() {
        let mut sa = ProductSpace::new(vec![Ordinal::new(2); 2]);
        let mut sb = ProductSpace::empty() + Ordinal::new(2) + Ordinal::new(2);

        assert_eq!(sa.dim(), sb.dim());
        assert_eq!(sa.card(), sb.card());

        sa = sa + Ordinal::new(3);
        sb = sb + Ordinal::new(3);

        assert_eq!(sa.dim(), Dim::Finite(3));
        assert_eq!(sa.dim(), sb.dim());

        assert_eq!(sa.card(), Card::Finite(12));
        assert_eq!(sa.card(), sb.card());

        let sc = sa + sb;

        assert_eq!(sc.dim(), Dim::Finite(6));
        assert_eq!(sc.card(), Card::Finite(144));
    }
}
