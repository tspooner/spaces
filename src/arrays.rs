use crate::{
    prelude::*,
    ops::{UnionPair, IntersectionPair},
};
use std::{iter::Map, convert::TryInto};
use itertools::{Itertools, structs::MultiProduct};

impl<const N: usize, D: Space> Space for [D; N] {
    type Value = [D::Value; N];

    fn is_empty(&self) -> bool { self.iter().any(|d| d.is_empty()) }

    fn contains(&self, val: &Self::Value) -> bool {
        self.iter().zip(val.iter()).all(|(d, x)| d.contains(x))
    }
}

impl<const N: usize, D: FiniteSpace> FiniteSpace for [D; N] {
    fn cardinality(&self) -> usize { self.iter().map(|d| d.cardinality()).product() }
}

impl<const N: usize, D: IterableSpace> IterableSpace for [D; N]
where
    D::Value: Clone,
    D::ElemIter: Clone,
{
    // TODO - Ideally, we would replace MultiProduct with an optimised implementation
    // for yielding arrays directly, not using an intermediate Vec.
    type ElemIter = Map<
        MultiProduct<D::ElemIter>,
        fn(Vec<D::Value>) -> [D::Value; N]
    >;

    fn elements(&self) -> Self::ElemIter {
        let iters: Vec<_> = self.iter().map(|s| s.elements()).collect();

        iters.into_iter().multi_cartesian_product().map(|x| {
            x.try_into().map_err(|_| ()).unwrap()
        })
    }
}

impl<const N: usize, D, S> Union<S> for [D; N]
where
    D: Space,
    S: Space<Value = [D::Value; N]>,
{
    type Output = UnionPair<Self, S>;

    fn union(self, rhs: S) -> Self::Output { UnionPair(self, rhs) }
}

impl<const N: usize, D, S> Intersection<S> for [D; N]
where
    D: Space,
    S: Space<Value = [D::Value; N]>,
{
    type Output = IntersectionPair<Self, S>;

    fn intersect(self, rhs: S) -> Option<Self::Output> {
        Some(IntersectionPair(self, rhs))
    }
}

impl<const N: usize, D: Closure> Closure for [D; N] {
    type Output = [D::Output; N];

    fn closure(self) -> Self::Output { self.map(|d| d.closure()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intervals::Interval;

    #[test]
    fn test_is_empty() {
        assert!([
            Interval::open_unchecked(0.0f64, 0.0),
            Interval::open_unchecked(1.0, 1.0),
        ].is_empty());

        assert!([
            Interval::open_unchecked(0.0f64, 0.0),
            Interval::open_unchecked(1.0, 2.0),
        ].is_empty());

        assert!(![
            Interval::degenerate(0.0f64),
            Interval::unit()
        ].is_empty());
    }

    #[test]
    fn test_contains() {
        let s = [
            Interval::degenerate(0.0f64),
            Interval::unit()
        ];

        for b in [0.0, 0.25, 0.5, 0.75, 1.0] {
            assert!(s.contains(&[0.0, b]));

            for a in [-1.0, 1.0] {
                assert!(!s.contains(&[a, b]));
            }
        }
    }

    #[test]
    fn test_card() {
        assert_eq!([
            Interval::degenerate(1usize),
            Interval::degenerate(0),
        ].cardinality(), 1);

        assert_eq!([
            Interval::lorc_unchecked(0usize, 2usize),
            Interval::lorc_unchecked(0usize, 2usize)
        ].cardinality(), 4);

        assert_eq!([
            Interval::closed_unchecked(0usize, 2usize),
            Interval::closed_unchecked(0, 100)
        ].cardinality(), 303);
    }

    #[test]
    fn test_values() {
        let space = [Interval::closed_unchecked(0, 1), Interval::closed_unchecked(2, 3)];
        let values: Vec<_> = space.elements().collect();

        assert_eq!(values, vec![
            [0, 2],
            [0, 3],
            [1, 2],
            [1, 3],
        ])
    }

    #[test]
    fn test_union() {
        let x = [
            Interval::closed_unchecked(0.0, 1.0),
            Interval::closed_unchecked(0.0, 1.0)
        ];
        let y = [
            Interval::closed_unchecked(5.0, 6.0),
            Interval::closed_unchecked(5.0, 6.0)
        ];
        let z = x.union(y);

        assert!(z.contains(&[0.0, 0.0]));
        assert!(z.contains(&[1.0, 1.0]));
        assert!(z.contains(&[5.0, 5.0]));
        assert!(z.contains(&[6.0, 6.0]));

        assert!(!z.contains(&[0.0, 5.0]));
        assert!(!z.contains(&[1.0, 6.0]));
    }
}
