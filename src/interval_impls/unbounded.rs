use crate::{ops, OrderedSpace, Space};

impl<V> Space for intervals::Unbounded<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, _: &V) -> bool { true }
}

impl<V: PartialOrd> OrderedSpace for intervals::Unbounded<V> {
    fn min(&self) -> Option<Self::Value> { None }

    fn inf(&self) -> Option<Self::Value> { None }

    fn max(&self) -> Option<Self::Value> { None }

    fn sup(&self) -> Option<Self::Value> { None }
}

impl<V> ops::Closure for intervals::Unbounded<V> {
    type Output = intervals::Unbounded<V>;

    fn closure(self) -> Self::Output { self }
}

///////////////////////////////////////////////////////////////////
// Union Implementations
///////////////////////////////////////////////////////////////////
macro_rules! impl_unbounded_union {
    ($v:ident; $type:ty) => {
        impl<$v: PartialOrd> ops::Union<$type> for intervals::Unbounded<$v> {
            type Output = intervals::Unbounded<$v>;

            fn union(self, _: $type) -> Self::Output { self }
        }

        impl<$v: PartialOrd> ops::Union<intervals::Unbounded<$v>> for $type {
            type Output = intervals::Unbounded<$v>;

            fn union(self, rhs: intervals::Unbounded<$v>) -> Self::Output { rhs }
        }
    };
}

impl_unbounded_union!(V; intervals::Closed<V>);
impl_unbounded_union!(V; intervals::Open<V>);

impl_unbounded_union!(V; intervals::LeftOpen<V>);
impl_unbounded_union!(V; intervals::LeftClosed<V>);

impl_unbounded_union!(V; intervals::RightOpen<V>);
impl_unbounded_union!(V; intervals::RightClosed<V>);

impl_unbounded_union!(V; intervals::LCRO<V>);
impl_unbounded_union!(V; intervals::LORC<V>);
