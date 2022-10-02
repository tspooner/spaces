//! Module for operations acting on spaces.
use crate::Space;

/// Trait for types that have a well-defined closure.
pub trait Closure: Space {
    type Output: Space<Value = Self::Value>;

    fn closure(self) -> Self::Output;
}

mod union;
pub use self::union::{Union, UnionOf, UnionPair};

mod intersection;
pub use self::intersection::{Intersection, IntersectionOf, IntersectionPair};

pub type UnionClosureOf<S, T> = <UnionOf<S, T> as Closure>::Output;

/// Trait for types implementing standard space operations.
pub trait Operations<S = Self>: Closure + Union<S> + Intersection<S>
where S: Space<Value = Self::Value>
{
    /// Compute the union-closure of the space.
    fn union_closure(self, rhs: S) -> UnionClosureOf<Self, S>
    where
        Self: Sized,
        UnionOf<Self, S>: Closure,
    {
        self.union(rhs).closure()
    }
}

impl<S, T> Operations<S> for T
where
    S: Space,
    T: Space<Value = S::Value> + Closure + Union<S> + Intersection<S>,

    <Self as Intersection<S>>::Output: Closure,
{
}
