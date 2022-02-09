use crate::prelude::*;
use std::{
    collections::HashMap,
    hash::Hash,
};

impl<K, D> Space for HashMap<K, D>
where
    K: Clone + Eq + Hash,
    D: Space,
{
    type Value = HashMap<K, D::Value>;

    fn dim(&self) -> Dim {
        self.values().fold(Dim::Finite(0), |acc, d| acc + d.dim())
    }

    fn card(&self) -> Card {
        self.values().fold(Card::Finite(0), |acc, d| acc * d.card())
    }

    fn contains(&self, val: &Self::Value) -> bool {
        val.iter().all(|(k, v)| match self.get(k) {
            Some(d) => d.contains(v),
            None => false,
        })
    }
}

impl<K, D> Union for HashMap<K, D>
where
    K: Clone + Eq + Hash,
    D: Clone + Space + Union,
{
    fn union(mut self, other: &Self) -> Self {
        for (k, v) in other.iter() {
            self.entry(k.clone()).and_modify(|d| {
                *d = d.clone().union(v)
            }).or_insert(v.clone());
        }

        self
    }
}

impl<K, D> Intersect for HashMap<K, D>
where
    K: Clone + Eq + Hash,
    D: Clone + Space + Intersect,
{
    fn intersect(mut self, other: &Self) -> Self {
        for (k, v) in other.iter() {
            self.entry(k.clone()).and_modify(|d| {
                *d = d.clone().intersect(v)
            }).or_insert(v.clone());
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Interval;
    use super::*;

    #[test]
    fn test_dim() {
        let space = HashMap::from([
            ("position", Interval::bounded(0.0, 1.0)),
            ("velocity", Interval::bounded(0.0, 1.0))
        ]);

        assert_eq!(space.dim(), Dim::Finite(2));
    }

    #[test]
    fn test_card() {
        let space = HashMap::from([
            ("position", 0..2),
            ("velocity", 0..5)
        ]);

        assert_eq!(space.card(), Card::Finite(10));
    }

    #[test]
    fn test_union() {
        let s1 = HashMap::from([
            ("position", Interval::bounded(0.0, 1.0)),
            ("velocity", Interval::bounded(0.0, 1.0))
        ]);
        let s2 = HashMap::from([
            ("position", Interval::bounded(-1.0, 1.0)),
            ("velocity", Interval::bounded(0.0, 0.5))
        ]);

        let s12 = s1.union(&s2);

        assert_eq!(s12["position"], Interval::bounded(-1.0, 1.0));
        assert_eq!(s12["velocity"], Interval::bounded(0.0, 1.0));
    }
}
