//! Module for operations acting on spaces.
use crate::Space;

type OoC<T> = crate::intervals::bounds::OpenOrClosed<T>;

enum LRB<T> { Left(T), Both(T), Right(T), }

impl<T> LRB<T> {
    fn translate(
        self,
        left: impl FnOnce(T) -> OoC<T>,
        both: impl FnOnce(T) -> OoC<T>,
        right: impl FnOnce(T) -> OoC<T>,
    ) -> OoC<T> {
        match self {
            LRB::Left(x) => left(x),
            LRB::Both(x) => both(x),
            LRB::Right(x) => right(x),
        }
    }
}

fn min_val<T: PartialOrd>(x: T, y: T) -> LRB<T> {
    if x < y { LRB::Left(x) } else if x == y { LRB::Both(x) } else { LRB::Right(y) }
}

fn max_val<T: PartialOrd>(x: T, y: T) -> LRB<T> {
    if x < y { LRB::Right(y) } else if x == y { LRB::Both(x) } else { LRB::Left(x) }
}

/// Trait for types that have a well-defined closure.
pub trait Closure: Space {
    type Output: Space<Value = Self::Value>;

    fn closure(self) -> Self::Output;
}

pub type ClosureOf<S> = <S as Closure>::Output;

mod union;
pub use self::union::{Union, UnionOf, UnionClosureOf, UnionPair};

mod intersection;
pub use self::intersection::{Intersection, IntersectionOf, IntersectionPair};
