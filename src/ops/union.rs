use crate::{
    intervals::{
        bounds::{self, Bound},
        Closed,
        Interval,
    },
    OrderedSpace,
    Space,
};

/// Trait for types that support the union operation.
///
/// The union of a collection of sets is the set containing all
/// such elements that are present in at least one set within the collection.
pub trait Union<Rhs: Space = Self>: Space {
    type Output: Space<Value = Self::Value>;

    fn union(self, rhs: Rhs) -> Self::Output;
}

pub type UnionOf<S, T> = <S as Union<T>>::Output;

/// Type representing the union of two spaces.
#[derive(Copy, Clone, Debug)]
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
    fn min(&self) -> Option<A::Value> {
        match (self.0.min(), self.1.min()) {
            (Some(x), Some(y)) => {
                if x <= y {
                    Some(x)
                } else {
                    Some(y)
                }
            },
            (x @ Some(_), None) => x,
            (None, y @ Some(_)) => y,
            _ => None,
        }
    }

    fn inf(&self) -> Option<A::Value> {
        match (self.0.inf(), self.1.inf()) {
            (Some(x), Some(y)) => {
                if x <= y {
                    Some(x)
                } else {
                    Some(y)
                }
            },
            (x @ Some(_), None) => x,
            (None, y @ Some(_)) => y,
            _ => None,
        }
    }

    fn max(&self) -> Option<A::Value> {
        match (self.0.max(), self.1.max()) {
            (Some(x), Some(y)) => {
                if x >= y {
                    Some(x)
                } else {
                    Some(y)
                }
            },
            (x @ Some(_), None) => x,
            (None, y @ Some(_)) => y,
            _ => None,
        }
    }

    fn sup(&self) -> Option<A::Value> {
        match (self.0.sup(), self.1.sup()) {
            (Some(x), Some(y)) => {
                if x >= y {
                    Some(x)
                } else {
                    Some(y)
                }
            },
            (x @ Some(_), None) => x,
            (None, y @ Some(_)) => y,
            _ => None,
        }
    }
}

impl<L, R, LL, RR> super::Closure for UnionPair<Interval<L, R>, Interval<LL, RR>>
where
    L: bounds::Pinch<LL>,
    R: bounds::Pinch<RR, Value = L::Value>,

    LL: bounds::Bound<Value = L::Value>,
    RR: bounds::Bound<Value = L::Value>,

    Closed<L::Value>: Space<Value = L::Value>,

    Interval<L, R>: Space<Value = L::Value>,
    Interval<LL, RR>: Space<Value = L::Value>,

    Interval<
        <<L as bounds::Pinch<LL>>::Down as Bound>::WithLimit,
        <<R as bounds::Pinch<RR>>::Up as Bound>::WithLimit,
    >: Space<Value = L::Value>,
{
    type Output = Interval<
        <<L as bounds::Pinch<LL>>::Down as Bound>::WithLimit,
        <<R as bounds::Pinch<RR>>::Up as Bound>::WithLimit,
    >;

    fn closure(self) -> Self::Output {
        Interval {
            left: self.0.left.pinch_down(self.1.left).with_limit_point(),
            right: self.0.right.pinch_up(self.1.right).with_limit_point(),
        }
    }
}

// impl<A, B, C> Intersect<C> for UnionPair<A, B>
// where
// // UnionPair<A, B>: Space
// A: Space + Intersect<B>,
// B: Space<Value = A::Value>,

// // This impl
// A: Intersect<C>,
// B: Intersect<C, Output = <A as Intersect<C>>::Output>,
// C: Space,

// B::Output: Intersect<B::Output, Output = B::Output>,
// {
// type Output = B::Output;

// fn intersect(self, rhs: C) -> Option<Self::Output> {
// let xx = self.0.intersect(rhs);
// let yy = self.1.intersect(rhs);

// match (xx, yy) {
// (Some(x), Some(y)) => x.intersect(y),
// (x @ Some(_), None) => x,
// (None, y @ Some(_)) => y,
// _ => None,
// }
// }
// }

// impl<A, B, C> Intersect<UnionPair<A, B>> for C
// where
// // UnionPair<A, B>: Space
// A: Space + Intersect<B>,
// B: Space<Value = A::Value>,

// // This impl
// A: Intersect<C>,
// B: Intersect<C, Output = <A as Intersect<C>>::Output>,
// C: Space,

// B::Output: Intersect<B::Output, Output = B::Output>,
// {
// type Output = B::Output;

// fn intersect(&self, rhs: &UnionPair<A, B>) -> Option<Self::Output> {
// let xx = rhs.0.intersect(self);
// let yy = rhs.1.intersect(self);

// match (xx, yy) {
// (Some(x), Some(y)) => x.intersect(&y),
// (x @ Some(_), None) => x,
// (None, y @ Some(_)) => y,
// _ => None,
// }
// }
// }

impl<S, T> std::fmt::Display for UnionPair<S, T>
where
    S: Space + std::fmt::Display,
    T: Space<Value = S::Value> + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \u{222A} {}", self.0, self.1)
    }
}
