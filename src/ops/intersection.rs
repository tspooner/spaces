use crate::Space;

/// Trait for types that support the intersection operation.
///
/// The intersection of a collection of sets is the set containing all
/// such elements that are present in each set within the collection.
pub trait Intersection<Rhs: Space = Self>: Space {
    type Output: Space<Value = Self::Value>;

    fn intersection(self, rhs: Rhs) -> Option<Self::Output>;
}

pub type IntersectionOf<S, T> = <S as Intersection<T>>::Output;

/// Type representing the intersection of two spaces.
#[derive(Copy, Clone, Debug)]
pub struct IntersectionPair<A: Space, B: Space<Value = A::Value>>(pub A, pub B);

impl<A, B> Space for IntersectionPair<A, B>
where
    A: Space,
    B: Space<Value = A::Value>,
{
    type Value = A::Value;

    fn is_empty(&self) -> bool { self.0.is_empty() || self.1.is_empty() }

    fn contains(&self, val: &A::Value) -> bool { self.0.contains(val) && self.1.contains(val) }
}
