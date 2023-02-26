use crate::{Space, OrderedSpace};
use super::{OoC, LRB, min_val, max_val, Union, UnionPair};

fn clip_ooc<T: PartialOrd>(x: OoC<T>, y: OoC<T>, cmp: impl Fn(T, T) -> LRB<T>) -> OoC<T> {
    use crate::intervals::bounds::OpenOrClosed::*;

    match (x, y) {
        (Open(x), Open(y)) => cmp(x, y).translate(Open, Open, Open),
        (Open(x), Closed(y)) => cmp(x, y).translate(Open, Open, Closed),
        (Closed(x), Open(y)) => cmp(x, y).translate(Closed, Open, Open),
        (Closed(x), Closed(y)) => cmp(x, y).translate(Closed, Closed, Closed),
    }
}

/// Trait for types that support the intersect operation.
///
/// The intersection of a collection of sets is the set containing all
/// such elements that are present in each set within the collection.
pub trait Intersection<Rhs: Space = Self>: Space {
    type Output: Space;

    fn intersect(self, rhs: Rhs) -> Option<Self::Output>;
}

pub type IntersectionOf<S, T> = <S as Intersection<T>>::Output;

// TODO - Add warning to docstring that explains why this type should be avoided.
//        Namely, that you can lead to panic! when calling is_empty().
/// Type representing the intersection of two spaces.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct IntersectionPair<A: Space, B: Space<Value = A::Value>>(pub A, pub B);

impl<A, B> Space for IntersectionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
{
    type Value = A::Value;

    fn is_empty(&self) -> bool {
        if self.0.is_empty() || self.1.is_empty() { return true; }

        panic!(
            "It's not currently possible to evaluate IntersectionPair::is_empty when neither \
            interior space are empty."
        )
    }

    fn contains(&self, val: &A::Value) -> bool { self.0.contains(val) && self.1.contains(val) }
}

impl<A, B> OrderedSpace for IntersectionPair<A, B>
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

impl<A, B, Rhs> Union<Rhs> for IntersectionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
    Rhs: Space<Value = A::Value>,
{
    type Output = UnionPair<Self, Rhs>;

    fn union(self, rhs: Rhs) -> Self::Output { UnionPair(self, rhs) }
}

impl<A, B, Rhs> Intersection<Rhs> for IntersectionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
    Rhs: Space<Value = A::Value>,
{
    type Output = IntersectionPair<Self, Rhs>;

    fn intersect(self, rhs: Rhs) -> Option<Self::Output> {
        if self.0.is_empty() || self.1.is_empty() { return None; }

        Some(IntersectionPair(self, rhs))
    }
}
