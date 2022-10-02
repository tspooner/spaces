use crate::{intervals::bounds::Pinch, ops, OrderedSpace, Space};

///////////////////////////////
// Left Closed
///////////////////////////////
impl<V: PartialOrd> Space for intervals::LeftClosed<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val >= &self.left.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::LeftClosed<V> {
    fn min(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn inf(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn max(&self) -> Option<Self::Value> { None }

    fn sup(&self) -> Option<Self::Value> { None }
}

impl<V: PartialOrd> ops::Closure for intervals::LeftClosed<V> {
    type Output = intervals::LeftClosed<V>;

    fn closure(self) -> Self::Output { self }
}

impl_union_pair!(V; intervals::LeftClosed<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::LeftClosed<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::LeftClosed<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::LeftClosed<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::LeftClosed<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::LeftClosed<V>, intervals::Closed<V>);
impl_union_pair!(V; intervals::LeftClosed<V>, intervals::Open<V>);

impl<V: PartialOrd> ops::Union<intervals::LeftClosed<V>> for intervals::LeftClosed<V> {
    type Output = intervals::LeftClosed<V>;

    fn union(self, rhs: intervals::LeftClosed<V>) -> Self::Output {
        intervals::Interval {
            left: self.left.pinch_down(rhs.left),
            right: intervals::bounds::NoBound::new(),
        }
    }
}

///////////////////////////////
// Left Open
///////////////////////////////
impl<V: PartialOrd> Space for intervals::LeftOpen<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val > &self.left.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::LeftOpen<V> {
    fn min(&self) -> Option<Self::Value> { None }

    fn inf(&self) -> Option<Self::Value> { Some(self.left.0.clone()) }

    fn max(&self) -> Option<Self::Value> { None }

    fn sup(&self) -> Option<Self::Value> { None }
}

impl<V: PartialOrd> ops::Closure for intervals::LeftOpen<V> {
    type Output = intervals::LeftClosed<V>;

    fn closure(self) -> Self::Output { intervals::Interval::left_closed(self.left.0) }
}

impl_union_pair!(V; intervals::LeftOpen<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::LeftOpen<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::LeftOpen<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::LeftOpen<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::LeftOpen<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::LeftOpen<V>, intervals::Closed<V>);
impl_union_pair!(V; intervals::LeftOpen<V>, intervals::Open<V>);

impl<V: PartialOrd> ops::Union<intervals::LeftOpen<V>> for intervals::LeftOpen<V> {
    type Output = intervals::LeftOpen<V>;

    fn union(self, rhs: intervals::LeftOpen<V>) -> Self::Output {
        intervals::Interval {
            left: self.left.pinch_down(rhs.left),
            right: intervals::bounds::NoBound::new(),
        }
    }
}

///////////////////////////////
// Right Closed
///////////////////////////////
impl<V: PartialOrd> Space for intervals::RightClosed<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val <= &self.right.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::RightClosed<V> {
    fn min(&self) -> Option<Self::Value> { None }

    fn inf(&self) -> Option<Self::Value> { None }

    fn max(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }

    fn sup(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }
}

impl<V: PartialOrd> ops::Closure for intervals::RightClosed<V> {
    type Output = intervals::RightClosed<V>;

    fn closure(self) -> Self::Output { self }
}

impl_union_pair!(V; intervals::RightClosed<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::RightClosed<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::RightClosed<V>, intervals::RightOpen<V>);
impl_union_pair!(V; intervals::RightClosed<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::RightClosed<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::RightClosed<V>, intervals::Closed<V>);
impl_union_pair!(V; intervals::RightClosed<V>, intervals::Open<V>);

impl<V: PartialOrd> ops::Union<intervals::RightClosed<V>> for intervals::RightClosed<V> {
    type Output = intervals::RightClosed<V>;

    fn union(self, rhs: intervals::RightClosed<V>) -> Self::Output {
        intervals::Interval {
            left: intervals::bounds::NoBound::new(),
            right: self.right.pinch_down(rhs.right),
        }
    }
}

///////////////////////////////
// Right Open
///////////////////////////////
impl<V: PartialOrd> Space for intervals::RightOpen<V> {
    type Value = V;

    fn is_empty(&self) -> bool { false }

    fn contains(&self, val: &V) -> bool { val < &self.right.0 }
}

impl<V: PartialOrd + Clone> OrderedSpace for intervals::RightOpen<V> {
    fn min(&self) -> Option<Self::Value> { None }

    fn inf(&self) -> Option<Self::Value> { None }

    fn max(&self) -> Option<Self::Value> { None }

    fn sup(&self) -> Option<Self::Value> { Some(self.right.0.clone()) }
}

impl<V: PartialOrd> ops::Closure for intervals::RightOpen<V> {
    type Output = intervals::RightClosed<V>;

    fn closure(self) -> Self::Output { intervals::Interval::right_closed(self.right.0) }
}

impl_union_pair!(V; intervals::RightOpen<V>, intervals::LeftOpen<V>);
impl_union_pair!(V; intervals::RightOpen<V>, intervals::LeftClosed<V>);
impl_union_pair!(V; intervals::RightOpen<V>, intervals::RightClosed<V>);
impl_union_pair!(V; intervals::RightOpen<V>, intervals::LCRO<V>);
impl_union_pair!(V; intervals::RightOpen<V>, intervals::LORC<V>);
impl_union_pair!(V; intervals::RightOpen<V>, intervals::Closed<V>);
impl_union_pair!(V; intervals::RightOpen<V>, intervals::Open<V>);

impl<V: PartialOrd> ops::Union<intervals::RightOpen<V>> for intervals::RightOpen<V> {
    type Output = intervals::RightOpen<V>;

    fn union(self, rhs: intervals::RightOpen<V>) -> Self::Output {
        intervals::Interval {
            left: intervals::bounds::NoBound::new(),
            right: self.right.pinch_down(rhs.right),
        }
    }
}
