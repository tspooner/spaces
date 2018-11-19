use continuous::Interval;
use core::{Space, Card, Surjection, Vector};
use discrete::Partition;
use std::{
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Add, Index},
    slice::Iter as SliceIter
};

/// N-dimensional homogeneous space.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LinearSpace<D: Space> {
    dimensions: Vec<D>,
    card: Card,
}

impl<D: Space> LinearSpace<D> {
    pub fn new(dimensions: Vec<D>) -> Self {
        let mut s = Self::empty();

        for d in dimensions {
            s = s.push(d);
        }

        s
    }

    pub fn empty() -> Self {
        LinearSpace {
            dimensions: vec![],
            card: Card::Null,
        }
    }

    pub fn push(mut self, d: D) -> Self {
        self.card = self.card * d.card();
        self.dimensions.push(d);

        self
    }

    pub fn iter(&self) -> SliceIter<D> { self.dimensions.iter() }
}

impl LinearSpace<Interval> {
    pub fn partitioned(self, density: usize) -> LinearSpace<Partition> {
        self.into_iter()
            .map(|d| Partition::from_interval(d, density))
            .collect()
    }
}

impl LinearSpace<Partition> {
    pub fn centres(&self) -> Vec<Vec<f64>> {
        self.dimensions.iter().map(|d| d.centres()).collect()
    }
}

impl<D: Space> Space for LinearSpace<D> {
    type Value = Vector<D::Value>;

    fn dim(&self) -> usize { self.dimensions.len() }

    fn card(&self) -> Card { self.card }
}

impl<D, X> Surjection<Vec<X>, Vec<D::Value>> for LinearSpace<D>
where D: Space + Surjection<X, <D as Space>::Value>
{
    fn map(&self, val: Vec<X>) -> Vec<D::Value> {
        self.dimensions
            .iter()
            .zip(val.into_iter())
            .map(|(d, v)| d.map(v))
            .collect()
    }
}

impl<D, X> Surjection<Vector<X>, Vector<D::Value>> for LinearSpace<D>
where D: Space + Surjection<X, <D as Space>::Value>,
      X: Clone
{
    fn map(&self, val: Vector<X>) -> Vector<D::Value> {
        self.dimensions
            .iter()
            .zip(val.into_iter())
            .map(|(d, v)| d.map(v.clone()))
            .collect()
    }
}

impl<D: Space> Index<usize> for LinearSpace<D> {
    type Output = D;

    fn index(&self, index: usize) -> &D { self.dimensions.index(index) }
}

impl<D: Space> FromIterator<D> for LinearSpace<D> {
    fn from_iter<I: IntoIterator<Item = D>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<D: Space> IntoIterator for LinearSpace<D> {
    type Item = D;
    type IntoIter = ::std::vec::IntoIter<D>;

    fn into_iter(self) -> Self::IntoIter { self.dimensions.into_iter() }
}

impl<D: Space> Add<D> for LinearSpace<D> {
    type Output = Self;

    fn add(self, rhs: D) -> Self::Output { self.push(rhs) }
}

impl<D: Space> Add<LinearSpace<D>> for LinearSpace<D> {
    type Output = Self;

    fn add(self, rhs: LinearSpace<D>) -> Self::Output {
        FromIterator::from_iter(self.into_iter().chain(rhs.into_iter()))
    }
}

impl<D: Space + Display> fmt::Display for LinearSpace<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for (i, v) in self.dimensions.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }

            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    extern crate ndarray;

    use continuous::Interval;
    use core::{Space, Card, Surjection};
    use discrete::Ordinal;
    use product::LinearSpace;
    use std::iter::FromIterator;

    #[test]
    fn test_dim() {
        assert_eq!(LinearSpace::new(vec![Ordinal::new(2); 2]).dim(), 2);
    }

    #[test]
    fn test_card() {
        assert_eq!(
            LinearSpace::new(vec![Ordinal::new(2); 2]).card(),
            Card::Finite(4)
        );
    }

    #[test]
    fn test_surjection() {
        let space = LinearSpace::new(vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)]);

        assert_eq!(space.map(vec![6.0, 0.0]), vec![5.0, 1.0]);
        assert_eq!(space.map(vec![2.5, 1.5]), vec![2.5, 1.5]);
        assert_eq!(space.map(vec![-1.0, 3.0]), vec![0.0, 2.0]);
    }

    #[test]
    fn test_indexing() {
        let dimensions = vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];
        let space = LinearSpace::from_iter(dimensions.iter().cloned());

        assert_eq!(space[0], dimensions[0]);
        assert_eq!(space[1], dimensions[1]);
    }

    #[test]
    fn test_iteration() {
        let dimensions = vec![Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];
        let space = LinearSpace::from_iter(dimensions.iter().cloned());

        assert_eq!(space.into_iter().collect::<Vec<Interval>>(), dimensions);
    }

    #[test]
    fn test_add_op() {
        let mut sa = LinearSpace::new(vec![Ordinal::new(2); 2]);
        let mut sb = LinearSpace::empty() + Ordinal::new(2) + Ordinal::new(2);

        assert_eq!(sa.dim(), sb.dim());
        assert_eq!(sa.card(), sb.card());

        sa = sa + Ordinal::new(3);
        sb = sb + Ordinal::new(3);

        assert_eq!(sa.dim(), 3);
        assert_eq!(sa.dim(), sb.dim());

        assert_eq!(sa.card(), Card::Finite(12));
        assert_eq!(sa.card(), sb.card());

        let sc = sa + sb;

        assert_eq!(sc.dim(), 6);
        assert_eq!(sc.card(), Card::Finite(144));
    }
}
