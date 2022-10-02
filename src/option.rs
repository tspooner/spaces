use crate::{FiniteSpace, OrderedSpace, Space};

impl<S: Space> Space for Option<S> {
    type Value = S::Value;

    fn is_empty(&self) -> bool { self.is_none() }

    fn contains(&self, value: &Self::Value) -> bool {
        self.as_ref().map_or(false, |s| s.contains(value))
    }
}

impl<S: OrderedSpace> OrderedSpace for Option<S>
where S::Value: PartialOrd
{
    fn min(&self) -> Option<Self::Value> { self.as_ref().and_then(|s| s.min()) }

    fn inf(&self) -> Option<Self::Value> { self.as_ref().and_then(|s| s.inf()) }

    fn max(&self) -> Option<Self::Value> { self.as_ref().and_then(|s| s.max()) }

    fn sup(&self) -> Option<Self::Value> { self.as_ref().and_then(|s| s.sup()) }
}

impl<S: FiniteSpace> FiniteSpace for Option<S> {
    fn cardinality(&self) -> usize { self.as_ref().map_or(0, |s| s.cardinality()) }
}
