use {Dimension, Space, Span, Surjection};
use dimensions::{Continuous, Partitioned};
use rand::ThreadRng;
use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;
use std::iter::FromIterator;
use std::ops::Add;

/// Named, N-dimensional homogeneous space.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NamedSpace<D: Dimension> {
    dimensions: HashMap<String, D>,
    span: Span,
}

impl<D: Dimension> NamedSpace<D> {
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
            span: Span::Null,
        }
    }

    pub fn push<S: Into<String>>(mut self, name: S, d: D) -> Self {
        self.span = self.span*d.span();
        self.dimensions.insert(name.into(), d);

        self
    }

    pub fn iter(&self) -> HashMapIter<String, D> {
        self.dimensions.iter()
    }
}

impl NamedSpace<Continuous> {
    pub fn partitioned(self, density: usize) -> NamedSpace<Partitioned> {
        self.into_iter()
            .map(|(name, d)| (name, Partitioned::from_continuous(d, density)))
            .collect()
    }
}

impl NamedSpace<Partitioned> {
    pub fn centres(&self) -> Vec<Vec<f64>> {
        self.dimensions
            .values()
            .map(|d| d.centres())
            .collect()
    }
}

impl<D: Dimension> Space for NamedSpace<D> {
    type Repr = Vec<D::Value>;

    fn dim(&self) -> usize {
        self.dimensions.len()
    }

    fn span(&self) -> Span {
        self.span
    }

    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr {
        self.dimensions.values().map(|d| d.sample(rng)).collect()
    }
}

impl<D, X> Surjection<Vec<X>, Vec<D::Value>> for NamedSpace<D>
where
    D: Dimension + Surjection<X, <D as Dimension>::Value>,
{
    fn map(&self, val: Vec<X>) -> Vec<D::Value> {
        self.dimensions.values().zip(val.into_iter()).map(|(d, v)| d.map(v)).collect()
    }
}

impl<D: Dimension> FromIterator<(String, D)> for NamedSpace<D> {
    fn from_iter<I: IntoIterator<Item = (String, D)>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<D: Dimension> IntoIterator for NamedSpace<D> {
    type Item = (String, D);
    type IntoIter = ::std::collections::hash_map::IntoIter<String, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

impl<D: Dimension> Add<NamedSpace<D>> for NamedSpace<D> {
    type Output = Self;

    fn add(self, rhs: NamedSpace<D>) -> Self::Output {
        FromIterator::from_iter(self.into_iter().chain(rhs.into_iter()))
    }
}
