use crate::{ops, FiniteSpace, OrderedSpace, Space};

///////////////////////////////////
// Closed
///////////////////////////////////
impl<V: PartialOrd> Space for intervals::Closed<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val >= &self.left.0 && val <= &self.right.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::Closed<V> {
    fn min(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn inf(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn max(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }

    fn sup(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }
}

impl<V> FiniteSpace for intervals::Closed<V>
where
    V: num_traits::PrimInt,
    <V as std::ops::Sub>::Output: num_traits::NumCast,
{
    fn cardinality(&self) -> usize {
        num_traits::NumCast::from(self.right.0 - self.left.0 + V::one()).unwrap()
    }
}

impl<V: PartialOrd> ops::Closure for intervals::Closed<V> {
    type Output = intervals::Closed<V>;

    fn closure(self) -> Self::Output { self }
}

impl_union_pair!(V; intervals::Closed<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::Open<V>);
impl_union_pair!(V; intervals::Closed<V>, intervals::Closed<V>);

///////////////////////////////////
// LCRO
///////////////////////////////////
impl<V: PartialOrd> Space for intervals::LCRO<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val >= &self.left.0 && val < &self.right.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::LCRO<V> {
    fn min(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn inf(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn max(&self) -> Option<Self::Value> { None }

    fn sup(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }
}

impl<V> FiniteSpace for intervals::LCRO<V>
where
    V: num_traits::PrimInt,
    <V as std::ops::Sub>::Output: num_traits::NumCast,
{
    fn cardinality(&self) -> usize {
        num_traits::NumCast::from(self.right.0 - self.left.0).unwrap()
    }
}

impl<V: PartialOrd> ops::Closure for intervals::LCRO<V> {
    type Output = intervals::Closed<V>;

    fn closure(self) -> Self::Output {
        intervals::Interval {
            left: self.left,
            right: intervals::bounds::Closed(self.right.0),
        }
    }
}

impl_union_pair!(V; intervals::LCRO<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::Open<V>);
impl_union_pair!(V; intervals::LCRO<V>, intervals::Closed<V>);

///////////////////////////////////
// LORC
///////////////////////////////////
impl<V: PartialOrd> Space for intervals::LORC<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val > &self.left.0 && val <= &self.right.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::LORC<V> {
    fn min(&self) -> Option<Self::Value> { None }

    fn inf(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn max(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }

    fn sup(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }
}

impl<V> FiniteSpace for intervals::LORC<V>
where
    V: num_traits::PrimInt,
    <V as std::ops::Sub>::Output: num_traits::NumCast,
{
    fn cardinality(&self) -> usize {
        num_traits::NumCast::from(self.right.0 - self.left.0).unwrap()
    }
}

impl<V: PartialOrd> ops::Closure for intervals::LORC<V> {
    type Output = intervals::Closed<V>;

    fn closure(self) -> Self::Output {
        intervals::Interval {
            left: intervals::bounds::Closed(self.left.0),
            right: self.right,
        }
    }
}

impl_union_pair!(V; intervals::LORC<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::Open<V>);
impl_union_pair!(V; intervals::LORC<V>, intervals::Closed<V>);

///////////////////////////////////
// Open
///////////////////////////////////
impl<V: PartialOrd> Space for intervals::Open<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val > &self.left.0 && val < &self.right.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::Open<V> {
    fn min(&self) -> Option<Self::Value> { None }

    fn inf(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn max(&self) -> Option<Self::Value> { None }

    fn sup(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }
}

impl<V> FiniteSpace for intervals::Open<V>
where
    V: num_traits::PrimInt,
    <V as std::ops::Sub>::Output: num_traits::NumCast,
{
    fn cardinality(&self) -> usize {
        num_traits::NumCast::from(self.right.0 - self.left.0 - V::one()).unwrap()
    }
}

impl<V: PartialOrd> ops::Closure for intervals::Open<V> {
    type Output = intervals::Closed<V>;

    fn closure(self) -> Self::Output {
        intervals::Interval {
            left: intervals::bounds::Closed(self.left.0),
            right: intervals::bounds::Closed(self.right.0),
        }
    }
}

impl_union_pair!(V; intervals::Open<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::Open<V>);
impl_union_pair!(V; intervals::Open<V>, intervals::Closed<V>);
