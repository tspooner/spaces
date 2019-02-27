use continuous::Interval;
use core::*;
use discrete::Partition;
use itertools::Itertools;
use std::{
    collections::hash_map::{
        HashMap,
        Keys as HashMapKeys,
        Iter as HashMapIter,
        IntoIter as HashMapIntoIter
    },
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Add, Index},
};

/// Named, N-dimensional homogeneous space.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedSpace<D> {
    dimensions: HashMap<String, D>,
    card: Card,
}

impl<D: Space> NamedSpace<D> {
    pub fn new<S: Into<String>>(dimensions: Vec<(S, D)>) -> Self {
        let mut s = Self::empty();

        for (name, d) in dimensions {
            s = s.push(name, d);
        }

        s
    }

    pub fn push<S: Into<String>>(mut self, name: S, d: D) -> Self {
        self.card = self.card * d.card();
        self.dimensions.insert(name.into(), d);

        self
    }
}

impl<D: Space> NamedSpace<D> {
    pub fn empty() -> Self {
        NamedSpace {
            dimensions: HashMap::new(),
            card: Card::Null,
        }
    }

    pub fn iter(&self) -> HashMapIter<String, D> { self.dimensions.iter() }

    pub fn into_iter(self) -> HashMapIntoIter<String, D> { self.dimensions.into_iter() }

    pub fn keys(&self) -> HashMapKeys<'_, String, D> { self.dimensions.keys() }
}

impl NamedSpace<Interval> {
    pub fn partitioned(self, density: usize) -> NamedSpace<Partition> {
        self.into_iter()
            .map(|(name, d)| (name, Partition::from_interval(d, density)))
            .collect()
    }
}

impl NamedSpace<Partition> {
    pub fn centres(&self) -> HashMap<String, Vec<f64>> {
        self.dimensions
            .iter()
            .map(|(k, d)| (k.clone(), d.centres()))
            .collect()
    }
}

impl<D: Space> Space for NamedSpace<D> {
    type Value = HashMap<String, D::Value>;

    fn dim(&self) -> usize { self.dimensions.len() }

    fn card(&self) -> Card { self.card }
}

impl<D: Space + Enclose + Clone + fmt::Debug> Enclose for NamedSpace<D> {
    fn enclose(self, other: &Self) -> Self {
        let mut ns = Self::empty();
        let grouped = self.iter()
            .chain(other.iter())
            .sorted_by(|(kr, _), (kl, _)| kl.cmp(kr))
            .group_by(|(k, _)| k.clone());

        for (k, g) in &grouped {
            let mut it = g.map(|(_, d)| d);
            let d = it.next().map(|d| it.fold(d.clone(), |l, r| dbg!(l.enclose(r)))).unwrap();

            ns = ns.push(k.clone(), d);
        }

        ns
    }
}

impl<D, X> Surjection<Vec<X>, HashMap<String, D::Value>> for NamedSpace<D>
where D: Space + Surjection<X, <D as Space>::Value>
{
    fn map(&self, val: Vec<X>) -> HashMap<String, D::Value> {
        self.dimensions
            .iter()
            .zip(val.into_iter())
            .map(|((k, d), v)| (k.clone(), d.map(v)))
            .collect()
    }
}

impl<D, X> Surjection<HashMap<String, X>, HashMap<String, D::Value>> for NamedSpace<D>
where D: Space + Surjection<X, <D as Space>::Value>
{
    fn map(&self, val: HashMap<String, X>) -> HashMap<String, D::Value> {
        val.into_iter()
            .map(|(k, v)| (k.clone(), self.dimensions[&k].map(v)))
            .collect()
    }
}

impl<S: Into<String>, D: Space> Index<S> for NamedSpace<D> {
    type Output = D;

    fn index(&self, index: S) -> &D { self.dimensions.index(&index.into()) }
}

impl<S: Into<String>, D: Space> FromIterator<(S, D)> for NamedSpace<D> {
    fn from_iter<I: IntoIterator<Item = (S, D)>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<D: Space> IntoIterator for NamedSpace<D> {
    type Item = (String, D);
    type IntoIter = ::std::collections::hash_map::IntoIter<String, D>;

    fn into_iter(self) -> Self::IntoIter { self.dimensions.into_iter() }
}

impl<D: Space> Add<NamedSpace<D>> for NamedSpace<D> {
    type Output = Self;

    fn add(self, rhs: NamedSpace<D>) -> Self::Output {
        FromIterator::from_iter(self.into_iter().chain(rhs.into_iter()))
    }
}

impl<D: Space + Display> fmt::Display for NamedSpace<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (k, v)) in self.dimensions.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }

            write!(f, "{}: {}", k, v)?;
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    extern crate ndarray;

    use continuous::Interval;
    use core::*;
    use discrete::Ordinal;
    use product::NamedSpace;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    fn test_dim() {
        assert_eq!(
            NamedSpace::new(vec![("D1", Ordinal::new(2)), ("D2", Ordinal::new(2))]).dim(),
            2
        );
    }

    #[test]
    fn test_card() {
        assert_eq!(
            NamedSpace::new(vec![("D1", Ordinal::new(2)), ("D2", Ordinal::new(2))]).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_enclose() {
        let s1 = NamedSpace::new(vec![
            ("D1", Interval::bounded(0.0, 5.0)),
            ("D2", Interval::bounded(1.0, 3.0)),
        ]);
        let s2 = NamedSpace::new(vec![
            ("D2", Interval::bounded(1.0, 2.0)),
            ("D1", Interval::bounded(-5.0, 0.0)),
            ("D3", Interval::bounded(-10.0, 1.0)),
        ]);

        assert_eq!(s1.enclose(&s2), NamedSpace::new(vec![
            ("D1", Interval::bounded(-5.0, 5.0)),
            ("D2", Interval::bounded(1.0, 3.0)),
            ("D3", Interval::bounded(-10.0, 1.0)),
        ]));
    }

    #[test]
    fn test_surjection() {
        let space = NamedSpace::new(vec![
            ("D1", Interval::bounded(0.0, 5.0)),
            ("D2", Interval::bounded(1.0, 2.0)),
        ]);

        fn make(vals: Vec<f64>) -> HashMap<String, f64> {
            let mut m = HashMap::new();

            m.insert("D1".to_string(), vals[0]);
            m.insert("D2".to_string(), vals[1]);

            m
        }

        assert_eq!(space.map(make(vec![6.0, 0.0])), make(vec![5.0, 1.0]));
        assert_eq!(space.map(make(vec![2.5, 1.5])), make(vec![2.5, 1.5]));
        assert_eq!(space.map(make(vec![-1.0, 3.0])), make(vec![0.0, 2.0]));
    }

    #[test]
    fn test_indexing() {
        let d1 = Interval::bounded(0.0, 5.0);
        let d2 = Interval::bounded(1.0, 2.0);

        let space = NamedSpace::from_iter(vec![("D1", d1.clone()), ("D2", d2.clone())]);

        assert_eq!(space["D1"], d1);
        assert_eq!(space["D2"], d2);
    }

    #[test]
    fn test_iteration() {
        let dimensions = vec![
            ("D1".to_string(), Interval::bounded(0.0, 5.0)),
            ("D2".to_string(), Interval::bounded(1.0, 2.0)),
        ];
        let space = NamedSpace::new(dimensions.clone());

        assert_eq!(
            space.into_iter().collect::<HashMap<String, Interval>>(),
            HashMap::from_iter(dimensions)
        );
    }
}
