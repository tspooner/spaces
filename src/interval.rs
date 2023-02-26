use crate::{
    Space, OrderedSpace, FiniteSpace,
    ops::{Union, UnionPair, Intersection, Closure}
};
use intervals::{Interval, bounds::{self, OpenOrClosed}};

///////////////////////////////////////////////////////////////////
// Core Implementations
///////////////////////////////////////////////////////////////////
impl<L, R> Space for Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    L::Value: Clone,
{
    type Value = L::Value;

    fn is_empty(&self) -> bool {
        match (self.left.value(), self.right.value()) {
            (Some(l), Some(r)) if !self.left.is_closed() && !self.right.is_closed() => l == r,
            _ => false,
        }
    }

    fn contains(&self, val: &L::Value) -> bool {
        use OpenOrClosed::*;

        let check_left = self.inf().map_or(true, |l| match l {
            Open(ref l) => val > l,
            Closed(ref l) => val >= l,
        });
        let check_right = self.sup().map_or(true, |r| match r {
            Open(ref r) => val < r,
            Closed(ref r) => val <= r,
        });

        check_left && check_right
    }
}

impl<L, R> OrderedSpace for Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    L::Value: Clone,
{
    fn inf(&self) -> Option<OpenOrClosed<Self::Value>> {
        self.left.value().cloned().map(|l| if self.left.is_open() {
            OpenOrClosed::Open(l)
        } else {
            OpenOrClosed::Closed(l)
        })
    }

    fn sup(&self) -> Option<OpenOrClosed<Self::Value>> {
        self.right.value().cloned().map(|r| if self.right.is_open() {
            OpenOrClosed::Open(r)
        } else {
            OpenOrClosed::Closed(r)
        })
    }
}

macro_rules! impl_fs {
    ($v:ident; $left:ty, $right:ty; |$me:ident| $inner:block) => {
        impl<$v> FiniteSpace for Interval<$left, $right>
        where
            $v: num_traits::PrimInt,
            <$v as std::ops::Sub>::Output: num_traits::NumCast,
        {
            fn cardinality(&$me) -> usize {
                num_traits::NumCast::from($inner).unwrap()
            }
        }
    }
}

#[inline]
fn card_oo<V: num_traits::PrimInt>(left: V, right: V) -> <V as std::ops::Sub>::Output {
    let d = right - left;

    if d <= V::one() { V::zero() } else { d }
}

#[inline]
fn card_co<V: num_traits::PrimInt>(left: V, right: V) -> <V as std::ops::Sub>::Output {
    right - left
}

#[inline]
fn card_cc<V: num_traits::PrimInt>(left: V, right: V) -> <V as std::ops::Sub>::Output {
    right - left + V::one()
}

impl_fs!(V; bounds::Closed<V>, bounds::Closed<V>; |self| { card_cc(self.left.0, self.right.0) });
impl_fs!(V; bounds::Closed<V>, bounds::Open<V>; |self| { card_co(self.left.0, self.right.0) });
impl_fs!(V; bounds::Closed<V>, bounds::OpenOrClosed<V>; |self| {
    match self.right {
        bounds::OpenOrClosed::Open(r) => card_co(self.left.0, r),
        bounds::OpenOrClosed::Closed(r) => card_cc(self.left.0, r),
    }
});

impl_fs!(V; bounds::Open<V>, bounds::Closed<V>; |self| { card_co(self.left.0, self.right.0) });
impl_fs!(V; bounds::Open<V>, bounds::Open<V>; |self| { card_oo(self.left.0, self.right.0) });
impl_fs!(V; bounds::Open<V>, bounds::OpenOrClosed<V>; |self| {
    match self.right {
        bounds::OpenOrClosed::Open(r) => card_oo(self.left.0, r),
        bounds::OpenOrClosed::Closed(r) => card_co(self.left.0, r),
    }
});

impl_fs!(V; bounds::OpenOrClosed<V>, bounds::Closed<V>; |self| {
    match self.left {
        bounds::OpenOrClosed::Open(l) => card_co(l, self.right.0),
        bounds::OpenOrClosed::Closed(l) => card_cc(l, self.right.0),
    }
});
impl_fs!(V; bounds::OpenOrClosed<V>, bounds::Open<V>; |self| {
    match self.left {
        bounds::OpenOrClosed::Open(l) => card_oo(l, self.right.0),
        bounds::OpenOrClosed::Closed(l) => card_co(l, self.right.0),
    }
});
impl_fs!(V; bounds::OpenOrClosed<V>, bounds::OpenOrClosed<V>; |self| {
    use intervals::bounds::OpenOrClosed::{Open, Closed};

    match (self.left, self.right) {
        (Open(l), Open(r)) => card_oo(l, r),
        (Closed(l), Open(r)) | (Open(l), Closed(r)) => card_co(l, r),
        (Closed(l), Closed(r)) => card_cc(l, r),
    }
});

///////////////////////////////////////////////////////////////////
// Op Implementations
///////////////////////////////////////////////////////////////////
impl<L, R, LL, RR> Union<Interval<LL, RR>> for Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    LL: bounds::Bound,
    RR: bounds::Bound<Value = LL::Value>,

    Interval<L, R>: Space<Value = L::Value>,
    Interval<LL, RR>: Space<Value = L::Value>,
{
    type Output = UnionPair<Interval<L, R>, Interval<LL, RR>>;

    fn union(self, rhs: Interval<LL, RR>) -> Self::Output { UnionPair(self, rhs) }
}

impl<L, R, LL, RR> Intersection<Interval<LL, RR>> for Interval<L, R>
where
    L: bounds::Pinch<LL>,
    R: bounds::Pinch<RR, Value = L::Value>,

    L::Value: PartialOrd,

    LL: bounds::Bound,
    RR: bounds::Bound<Value = LL::Value>,

    Interval<L, R>: crate::Space<Value = L::Value>,
    Interval<LL, RR>: crate::Space<Value = LL::Value>,

    intervals::IntersectionOf<L, R, LL, RR>: crate::Space<Value = L::Value>,
    bounds::Validator: bounds::ValidateBounds<L::Left, R::Right>,
{
    type Output = intervals::IntersectionOf<L, R, LL, RR>;

    fn intersect(self, rhs: Interval<LL, RR>) -> Option<Self::Output> {
        Interval::intersect(self, rhs)
    }
}

impl<L, R> Closure for Interval<L, R>
where
    L: bounds::Bound,
    R: bounds::Bound<Value = L::Value>,

    Interval<L, R>: Space<Value = L::Value>,
    Interval<L::WithLimit, R::WithLimit>: Space<Value = L::Value>,
{
    type Output = Interval<L::WithLimit, R::WithLimit>;

    fn closure(self) -> Self::Output {
        Interval {
            left: self.left.with_limit_point(),
            right: self.right.with_limit_point(),
        }
    }
}

impl<L, R, LL, RR> Closure for UnionPair<Interval<L, R>, Interval<LL, RR>>
where
    L: bounds::Unroll<LL>,
    R: bounds::Unroll<RR, Value = L::Value>,

    LL: bounds::Bound<Value = L::Value>,
    RR: bounds::Bound<Value = LL::Value>,

    L::Value: Clone,
{
    type Output = crate::intervals::UnionClosureOf<L, R, LL, RR>;

    fn closure(self) -> Self::Output {
        Interval::union_closure(self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed_intersection() {
        let a = Interval::closed_unchecked(0.0, 1.0);
        let b = Interval::closed_unchecked(1.0, 2.0);
        let c = Interval::closed_unchecked(2.0, 3.0);

        assert_eq!(a.intersect(a).unwrap(), a);

        assert_eq!(a.intersect(b).unwrap(), Interval::degenerate(1.0));
        assert_eq!(b.intersect(c).unwrap(), Interval::degenerate(2.0));

        assert_eq!(a.intersect(c), None);
    }
}
