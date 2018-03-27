use {Space, Span, Surjection};
use dimensions::{Continuous, Partitioned};
use rand::ThreadRng;
use std::iter::FromIterator;
use std::slice::Iter as SliceIter;
use std::ops::{Add, Index};

/// N-dimensional homogeneous space.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RegularSpace<D: Space> {
    dimensions: Vec<D>,
    span: Span,
}

impl<D: Space> RegularSpace<D> {
    pub fn new(dimensions: Vec<D>) -> Self {
        let mut s = Self::empty();

        for d in dimensions {
            s = s.push(d);
        }

        s
    }

    pub fn empty() -> Self {
        RegularSpace {
            dimensions: vec![],
            span: Span::Null,
        }
    }

    pub fn push(mut self, d: D) -> Self {
        self.span = self.span*d.span();
        self.dimensions.push(d);

        self
    }

    pub fn iter(&self) -> SliceIter<D> {
        self.dimensions.iter()
    }
}

impl RegularSpace<Continuous> {
    pub fn partitioned(self, density: usize) -> RegularSpace<Partitioned> {
        self.into_iter()
            .map(|d| Partitioned::from_continuous(d, density))
            .collect()
    }
}

impl RegularSpace<Partitioned> {
    pub fn centres(&self) -> Vec<Vec<f64>> {
        self.dimensions
            .iter()
            .map(|d| d.centres())
            .collect()
    }
}

impl<D: Space> Space for RegularSpace<D> {
    type Value = Vec<D::Value>;

    fn dim(&self) -> usize {
        self.dimensions.len()
    }

    fn span(&self) -> Span {
        self.span
    }

    fn sample(&self, rng: &mut ThreadRng) -> Vec<D::Value> {
        self.dimensions.iter().map(|d| d.sample(rng)).collect()
    }
}

impl<D, X> Surjection<Vec<X>, Vec<D::Value>> for RegularSpace<D>
where
    D: Space + Surjection<X, <D as Space>::Value>,
{
    fn map(&self, val: Vec<X>) -> Vec<D::Value> {
        self.dimensions.iter().zip(val.into_iter()).map(|(d, v)| d.map(v)).collect()
    }
}

impl<D: Space> Index<usize> for RegularSpace<D> {
    type Output = D;

    fn index(&self, index: usize) -> &D {
        self.dimensions.index(index)
    }
}

impl<D: Space> FromIterator<D> for RegularSpace<D> {
    fn from_iter<I: IntoIterator<Item = D>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<D: Space> IntoIterator for RegularSpace<D> {
    type Item = D;
    type IntoIter = ::std::vec::IntoIter<D>;

    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

impl<D: Space> Add<D> for RegularSpace<D> {
    type Output = Self;

    fn add(self, rhs: D) -> Self::Output {
        self.push(rhs)
    }
}

impl<D: Space> Add<RegularSpace<D>> for RegularSpace<D> {
    type Output = Self;

    fn add(self, rhs: RegularSpace<D>) -> Self::Output {
        FromIterator::from_iter(self.into_iter().chain(rhs.into_iter()))
    }
}


#[cfg(test)]
mod tests {
    use {Space, RegularSpace, Span, Surjection};
    use dimensions::{Discrete, Continuous};
    use ndarray::arr1;
    use rand::thread_rng;
    use std::iter::FromIterator;

    #[test]
    fn test_dim() {
        assert_eq!(RegularSpace::new(vec![Discrete::new(2); 2]).dim(), 2);
    }

    #[test]
    fn test_span() {
        assert_eq!(RegularSpace::new(vec![Discrete::new(2); 2]).span(), Span::Finite(4));
    }

    #[test]
    fn test_sampling() {
        let space = RegularSpace::new(vec![Discrete::new(2); 2]);

        let mut rng = thread_rng();

        let mut c1 = arr1(&vec![0.0; 2]);
        let mut c2 = arr1(&vec![0.0; 2]);
        for _ in 0..5000 {
            let sample = space.sample(&mut rng);

            c1[sample[0]] += 1.0;
            c2[sample[1]] += 1.0;

            assert!(sample[0] == 0 || sample[0] == 1);
            assert!(sample[1] == 0 || sample[1] == 1);
        }

        assert!((c1/5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
        assert!((c2/5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
    }

    #[test]
    fn test_surjection() {
        let space = RegularSpace::new(vec![Continuous::new(0.0, 5.0), Continuous::new(1.0, 2.0)]);

        assert_eq!(space.map(vec![6.0, 0.0]), vec![5.0, 1.0]);
        assert_eq!(space.map(vec![2.5, 1.5]), vec![2.5, 1.5]);
        assert_eq!(space.map(vec![-1.0, 3.0]), vec![0.0, 2.0]);
    }

    #[test]
    fn test_indexing() {
        let dimensions = vec![Continuous::new(0.0, 5.0), Continuous::new(1.0, 2.0)];
        let space = RegularSpace::from_iter(dimensions.iter().cloned());

        assert_eq!(space[0], dimensions[0]);
        assert_eq!(space[1], dimensions[1]);
    }

    #[test]
    fn test_iteration() {
        let dimensions = vec![Continuous::new(0.0, 5.0), Continuous::new(1.0, 2.0)];
        let space = RegularSpace::from_iter(dimensions.iter().cloned());

        assert_eq!(space.into_iter().collect::<Vec<Continuous>>(), dimensions);
    }

    #[test]
    fn test_add_op() {
        let mut sa = RegularSpace::new(vec![Discrete::new(2); 2]);
        let mut sb = RegularSpace::empty() + Discrete::new(2) + Discrete::new(2);

        assert_eq!(sa.dim(), sb.dim());
        assert_eq!(sa.span(), sb.span());

        sa = sa + Discrete::new(3);
        sb = sb + Discrete::new(3);

        assert_eq!(sa.dim(), 3);
        assert_eq!(sa.dim(), sb.dim());

        assert_eq!(sa.span(), Span::Finite(12));
        assert_eq!(sa.span(), sb.span());

        let sc = sa + sb;

        assert_eq!(sc.dim(), 6);
        assert_eq!(sc.span(), Span::Finite(144));
    }
}
