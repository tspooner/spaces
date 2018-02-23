use super::*;

/// N-dimensional homogeneous space.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RegularSpace<D: Dimension> {
    dimensions: Vec<D>,
    span: Span,
}

impl<D: Dimension> RegularSpace<D> {
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

impl RegularSpace<dimensions::Continuous> {
    pub fn partitioned(self, density: usize) -> RegularSpace<Partitioned> {
        self.into_iter()
            .map(|d| Partitioned::from_continuous(d, density))
            .collect()
    }
}

impl RegularSpace<dimensions::Partitioned> {
    pub fn centres(&self) -> Vec<Vec<f64>> {
        self.dimensions
            .iter()
            .map(|d| d.centres())
            .collect()
    }
}

impl<D: Dimension> Space for RegularSpace<D> {
    type Repr = Vec<D::Value>;

    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr {
        self.dimensions.iter().map(|d| d.sample(rng)).collect()
    }

    fn dim(&self) -> usize {
        self.dimensions.len()
    }

    fn span(&self) -> Span {
        self.span
    }
}

impl<D: Dimension> FromIterator<D> for RegularSpace<D> {
    fn from_iter<I: IntoIterator<Item = D>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<D: Dimension> IntoIterator for RegularSpace<D> {
    type Item = D;
    type IntoIter = ::std::vec::IntoIter<D>;

    fn into_iter(self) -> Self::IntoIter {
        self.dimensions.into_iter()
    }
}

impl<D: Dimension> Add<D> for RegularSpace<D> {
    type Output = Self;

    fn add(self, rhs: D) -> Self::Output {
        self.push(rhs)
    }
}

impl<D: Dimension> Add<RegularSpace<D>> for RegularSpace<D> {
    type Output = Self;

    fn add(self, rhs: RegularSpace<D>) -> Self::Output {
        FromIterator::from_iter(self.into_iter().chain(rhs.into_iter()))
    }
}

impl<D: Dimension> Index<usize> for RegularSpace<D> {
    type Output = D;

    fn index(&self, index: usize) -> &D {
        self.dimensions.index(index)
    }
}


#[cfg(test)]
mod tests {
    use ndarray::arr1;
    use rand::thread_rng;
    use spaces::{Space, RegularSpace, Span};
    use spaces::dimensions::Discrete;

    #[test]
    fn test_regular_space() {
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

        assert_eq!(space.dim(), 2);
        assert_eq!(space.span(), Span::Finite(4));
    }

    #[test]
    fn test_regular_space_sugar() {
        let mut sa = RegularSpace::new(vec![Discrete::new(2); 2]);
        let mut sb = RegularSpace::empty() + Discrete::new(2) + Discrete::new(2);

        assert_eq!(sa.dim(), sb.dim());
        assert_eq!(sa.span(), sb.span());

        sa = sa + Discrete::new(3);
        sb = sb + Discrete::new(3);

        assert_eq!(sa.dim(), 3);
        assert_eq!(sa.dim(), sb.dim());

        assert_eq!(sa.span(), Span::Finite(4)*Span::Finite(3));
        assert_eq!(sa.span(), sb.span());
    }
}
