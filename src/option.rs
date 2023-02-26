use crate::{
    FiniteSpace, OrderedSpace, Space,
    intervals::bounds::OpenOrClosed,
    ops::UnionPair,
    prelude::*,
};

impl<S: Space> Space for Option<S> {
    type Value = S::Value;

    fn is_empty(&self) -> bool { self.as_ref().map_or(true, |s| s.is_empty()) }

    fn contains(&self, value: &Self::Value) -> bool {
        self.as_ref().map_or(false, |s| s.contains(value))
    }
}

impl<S: OrderedSpace> OrderedSpace for Option<S>
where S::Value: PartialOrd
{
    fn inf(&self) -> Option<OpenOrClosed<Self::Value>> {
        self.as_ref().and_then(|s| s.inf())
    }

    fn sup(&self) -> Option<OpenOrClosed<Self::Value>> {
        self.as_ref().and_then(|s| s.sup())
    }
}

impl<S: FiniteSpace> FiniteSpace for Option<S> {
    fn cardinality(&self) -> usize { self.as_ref().map_or(0, |s| s.cardinality()) }
}

impl<S: Closure> Closure for Option<S> {
    type Output = Option<S::Output>;

    fn closure(self) -> Self::Output {
        self.map(|s| s.closure())
    }
}

impl<S: Space, T: Space<Value = S::Value>> Union<T> for Option<S> {
    type Output = UnionPair<Self, T>;

    fn union(self, rhs: T) -> Self::Output { UnionPair(self, rhs) }
}

impl<S: Intersection<T>, T: Space<Value = S::Value>> Intersection<T> for Option<S> {
    type Output = S::Output;

    fn intersect(self, rhs: T) -> Option<Self::Output> {
        self.and_then(|s| s.intersect(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intervals::Interval;

    #[test]
    fn test_is_empty() {
        let s: Option<crate::intervals::Open<f64>> = None;

        assert!(s.is_empty());
        assert!(Some(Interval::open_unchecked(0.0f64, 0.0)).is_empty());
        assert!(!Some(Interval::degenerate(0.0)).is_empty());
    }

    #[test]
    fn test_union() {
        let e: Option<crate::real::PositiveReals<f64>> = None;
        let a = e.union(Interval::degenerate(0.0));
        let b = Some(crate::real::positive_reals()).union(Interval::degenerate(0.0));

        assert!(!a.contains(&-1.0));
        assert!(!b.contains(&-1.0));

        assert!(a.contains(&0.0));
        assert!(b.contains(&0.0));

        assert!(!a.contains(&1.0));
        assert!(b.contains(&1.0));
    }

    #[test]
    fn test_intersect() {
        let e: Option<crate::real::PositiveReals<f64>> = None;
        let a = e.intersect(Interval::degenerate(0.0));
        let b = Some(crate::real::positive_reals()).intersect(Interval::degenerate(0.0));

        assert!(a.is_none());
        assert!(b.is_none());
    }
}
