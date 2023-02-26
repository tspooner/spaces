use crate::{OrderedSpace, Space};
use super::{OoC, LRB, min_val, max_val, Intersection, IntersectionPair, Closure, ClosureOf};

fn clip_ooc<T: PartialOrd>(x: OoC<T>, y: OoC<T>, cmp: impl Fn(T, T) -> LRB<T>) -> OoC<T> {
    use crate::intervals::bounds::OpenOrClosed::*;

    match (x, y) {
        (Open(x), Open(y)) => cmp(x, y).translate(Open, Open, Open),
        (Open(x), Closed(y)) => cmp(x, y).translate(Open, Closed, Closed),
        (Closed(x), Open(y)) => cmp(x, y).translate(Closed, Closed, Open),
        (Closed(x), Closed(y)) => cmp(x, y).translate(Closed, Closed, Closed),
    }
}

/// Trait for types that support the union operation.
///
/// The union of a collection of sets is the set containing all
/// such elements that are present in at least one set within the collection.
pub trait Union<Rhs: Space = Self>: Space {
    type Output: Space;

    fn union(self, rhs: Rhs) -> Self::Output;

    /// Compute the union-closure of the space.
    fn union_closure(self, rhs: Rhs) -> UnionClosureOf<Self, Rhs>
    where
        Self: Sized,
        Self::Output: Closure,
    {
        self.union(rhs).closure()
    }
}

pub type UnionOf<S, T> = <S as Union<T>>::Output;
pub type UnionClosureOf<S, T> = ClosureOf<UnionOf<S, T>>;

/// Type representing the union of two arbitrary spaces.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct UnionPair<A: Space, B: Space<Value = A::Value>>(pub A, pub B);

impl<A, B> Space for UnionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
{
    type Value = A::Value;

    fn is_empty(&self) -> bool { self.0.is_empty() && self.1.is_empty() }

    fn contains(&self, val: &A::Value) -> bool { self.0.contains(val) || self.1.contains(val) }
}

impl<A, B> OrderedSpace for UnionPair<A, B>
where
    A: OrderedSpace,
    B: OrderedSpace<Value = A::Value>,

    A::Value: PartialOrd,
{
    fn inf(&self) -> Option<OoC<A::Value>> {
        match (self.0.inf(), self.1.inf()) {
            (Some(left), Some(right)) => Some(clip_ooc(left, right, min_val)),
            _ => None,
        }
    }

    fn sup(&self) -> Option<OoC<A::Value>> {
        match (self.0.sup(), self.1.sup()) {
            (Some(left), Some(right)) => Some(clip_ooc(left, right, max_val)),
            _ => None,
        }
    }
}

impl<A, B, C> Union<C> for UnionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
    C: Space<Value = A::Value>,
{
    type Output = UnionPair<Self, C>;

    fn union(self, rhs: C) -> Self::Output { UnionPair(self, rhs) }
}

impl<A, B, C> Intersection<C> for UnionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
    C: Space<Value = A::Value>,
{
    type Output = IntersectionPair<Self, C>;

    fn intersect(self, rhs: C) -> Option<Self::Output> {
        let intersect = IntersectionPair(self, rhs);

        if intersect.is_empty() { None } else { Some(intersect) }
    }
}

impl<S, T> std::fmt::Display for UnionPair<S, T>
where
    S: Space + std::fmt::Display,
    T: Space<Value = S::Value> + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \u{222A} {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::intervals::Interval;
    use super::*;

    #[test]
    fn test_up_lower() {
        let pair = UnionPair(
            Interval::left_open(-1.0),
            Interval::left_closed(0.0)
        );

        assert_eq!(pair.inf().unwrap(), OoC::Open(-1.0));
        assert!(pair.sup().is_none());

        let pair = UnionPair(
            Interval::left_closed(0.0),
            Interval::left_open(-1.0)
        );

        assert_eq!(pair.inf().unwrap(), OoC::Open(-1.0));
        assert!(pair.sup().is_none());
    }

    #[test]
    fn test_up_upper() {
        let pair = UnionPair(
            Interval::right_open(1.0),
            Interval::right_closed(0.0)
        );

        assert_eq!(pair.sup().unwrap(), OoC::Open(1.0));
        assert!(pair.inf().is_none());

        let pair = UnionPair(
            Interval::right_closed(0.0),
            Interval::right_open(1.0)
        );

        assert_eq!(pair.sup().unwrap(), OoC::Open(1.0));
        assert!(pair.inf().is_none());
    }

    #[test]
    fn test_up_both() {
        let pair = UnionPair(
            Interval::open_unchecked(0.0, 1.0),
            Interval::closed_unchecked(-1.0, 1.0)
        );

        assert_eq!(pair.inf().unwrap(), OoC::Closed(-1.0));
        assert_eq!(pair.sup().unwrap(), OoC::Closed(1.0));

        let pair = UnionPair(
            Interval::open_unchecked(-1.0, 1.0),
            Interval::closed_unchecked(0.0, 1.0)
        );

        assert_eq!(pair.inf().unwrap(), OoC::Open(-1.0));
        assert_eq!(pair.sup().unwrap(), OoC::Closed(1.0));
    }
}
