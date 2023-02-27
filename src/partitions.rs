use crate::{
    intervals::{partitions::{self, Partition}, bounds::OpenOrClosed},
    FiniteSpace,
    OrderedSpace,
    Space,
    IterableSpace,
};
use std::ops::Range;

impl<V> Space for partitions::Uniform<V> {
    type Value = usize;

    fn is_empty(&self) -> bool { self.size > 0 }

    fn contains(&self, value: &usize) -> bool { value >= &0 && value < &self.size }
}

impl<V> OrderedSpace for partitions::Uniform<V> {
    fn inf(&self) -> Option<OpenOrClosed<usize>> {
        Some(OpenOrClosed::Closed(0))
    }

    fn sup(&self) -> Option<OpenOrClosed<usize>> {
        Some(OpenOrClosed::Closed(self.size - 1))
    }
}

impl<V> FiniteSpace for partitions::Uniform<V> {
    fn cardinality(&self) -> usize { self.size }
}

impl<V> IterableSpace for partitions::Uniform<V> {
    type ValueIter = Range<usize>;

    fn values(&self) -> Self::ValueIter { 0..self.size }
}

impl<const N: usize, V: PartialOrd + Clone> Space for partitions::Declarative<N, V> {
    type Value = usize;

    fn is_empty(&self) -> bool { self.len() > 0 }

    fn contains(&self, value: &usize) -> bool { value >= &0 && value < &self.len() }
}

impl<const N: usize, V: PartialOrd + Clone> OrderedSpace for partitions::Declarative<N, V> {
    fn inf(&self) -> Option<OpenOrClosed<usize>> {
        Some(OpenOrClosed::Closed(0))
    }

    fn sup(&self) -> Option<OpenOrClosed<usize>> {
        Some(OpenOrClosed::Closed(self.len() - 1))
    }
}

impl<const N: usize, V: PartialOrd + Clone> FiniteSpace for partitions::Declarative<N, V> {
    fn cardinality(&self) -> usize { self.len() }
}

impl<const N: usize, V: PartialOrd + Clone> IterableSpace for partitions::Declarative<N, V> {
    type ValueIter = Range<usize>;

    fn values(&self) -> Self::ValueIter { 0..N }
}
