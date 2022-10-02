use crate::{
    intervals::partitions::{self, Partition},
    FiniteSpace,
    OrderedSpace,
    Space,
};

impl<V> Space for partitions::Uniform<V> {
    type Value = usize;

    fn is_empty(&self) -> bool { self.size > 0 }

    fn contains(&self, value: &usize) -> bool { value >= &0 && value < &self.size }
}

impl<V> OrderedSpace for partitions::Uniform<V> {
    fn min(&self) -> Option<usize> { Some(0) }

    fn max(&self) -> Option<usize> { Some(self.size - 1) }
}

impl<V> FiniteSpace for partitions::Uniform<V> {
    fn cardinality(&self) -> usize { self.size }
}

impl<const N: usize, V: PartialOrd + Clone> Space for partitions::Declarative<N, V> {
    type Value = usize;

    fn is_empty(&self) -> bool { self.len() > 0 }

    fn contains(&self, value: &usize) -> bool { value >= &0 && value < &self.len() }
}

impl<const N: usize, V: PartialOrd + Clone> OrderedSpace for partitions::Declarative<N, V> {
    fn min(&self) -> Option<usize> { Some(0) }

    fn max(&self) -> Option<usize> { Some(self.len() - 1) }
}

impl<const N: usize, V: PartialOrd + Clone> FiniteSpace for partitions::Declarative<N, V> {
    fn cardinality(&self) -> usize { self.len() }
}
