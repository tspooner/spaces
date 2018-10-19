use continuous::Interval;
use core::{Space, Card, Surjection};
use discrete::Partition;
use rand::Rng;
use std::{
    collections::hash_map::{HashMap, Iter as HashMapIter},
    iter::FromIterator,
    ops::{Add, Index},
};

/// Named, N-dimensional homogeneous space.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NamedSpace<D: Space> {
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

    pub fn empty() -> Self {
        NamedSpace {
            dimensions: HashMap::new(),
            card: Card::Null,
        }
    }

    pub fn push<S: Into<String>>(mut self, name: S, d: D) -> Self {
        self.card = self.card * d.card();
        self.dimensions.insert(name.into(), d);

        self
    }

    pub fn iter(&self) -> HashMapIter<String, D> { self.dimensions.iter() }
}

impl NamedSpace<Interval> {
    pub fn partitioned(self, density: usize) -> NamedSpace<Partition> {
        self.into_iter()
            .map(|(name, d)| (name, Partition::from_continuous(d, density)))
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

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HashMap<String, D::Value> {
        self.dimensions
            .iter()
            .map(|(k, d)| (k.clone(), d.sample(rng)))
            .collect()
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

#[cfg(test)]
mod tests {
    extern crate ndarray;

    use continuous::Interval;
    use core::{Space, Card, Surjection};
    use discrete::Discrete;
    use product::NamedSpace;
    use rand::thread_rng;
    use self::ndarray::arr1;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    fn test_dim() {
        assert_eq!(
            NamedSpace::new(vec![("D1", Discrete::new(2)), ("D2", Discrete::new(2))]).dim(),
            2
        );
    }

    #[test]
    fn test_card() {
        assert_eq!(
            NamedSpace::new(vec![("D1", Discrete::new(2)), ("D2", Discrete::new(2))]).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_sampling() {
        let space = NamedSpace::new(vec![("D1", Discrete::new(2)), ("D2", Discrete::new(2))]);

        let mut rng = thread_rng();

        let mut c1 = arr1(&vec![0.0; 2]);
        let mut c2 = arr1(&vec![0.0; 2]);
        for _ in 0..5000 {
            let sample = space.sample(&mut rng);

            c1[sample["D1"]] += 1.0;
            c2[sample["D2"]] += 1.0;

            assert!(sample["D1"] == 0 || sample["D1"] == 1);
            assert!(sample["D2"] == 0 || sample["D2"] == 1);
        }

        assert!((c1 / 5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
        assert!((c2 / 5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
    }

    #[test]
    fn test_surjection() {
        let space = NamedSpace::new(vec![
            ("D1", Interval::closed(0.0, 5.0)),
            ("D2", Interval::closed(1.0, 2.0)),
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
        let d1 = Interval::closed(0.0, 5.0);
        let d2 = Interval::closed(1.0, 2.0);

        let space = NamedSpace::from_iter(vec![("D1", d1.clone()), ("D2", d2.clone())]);

        assert_eq!(space["D1"], d1);
        assert_eq!(space["D2"], d2);
    }

    #[test]
    fn test_iteration() {
        let dimensions = vec![
            ("D1".to_string(), Interval::closed(0.0, 5.0)),
            ("D2".to_string(), Interval::closed(1.0, 2.0)),
        ];
        let space = NamedSpace::new(dimensions.clone());

        assert_eq!(
            space.into_iter().collect::<HashMap<String, Interval>>(),
            HashMap::from_iter(dimensions)
        );
    }
}
