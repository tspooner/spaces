use super::*;

/// 1-dimensional space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct UnitarySpace<D: Dimension>(D);

impl<D: Dimension> UnitarySpace<D> {
    pub fn new(d: D) -> Self {
        UnitarySpace(d)
    }
}

impl UnitarySpace<dimensions::Continuous> {
    pub fn partitioned(self, density: usize) -> UnitarySpace<Partitioned> {
        UnitarySpace(Partitioned::from_continuous(self.0, density))
    }
}

impl<D: Dimension> Space for UnitarySpace<D> {
    type Repr = D::Value;

    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr {
        self.0.sample(rng)
    }

    fn dim(&self) -> usize {
        1
    }

    fn span(&self) -> Span {
        self.0.span()
    }
}


#[cfg(test)]
mod tests {
    use ndarray::arr1;
    use rand::thread_rng;
    use spaces::{Space, UnitarySpace, Span};
    use spaces::dimensions::Discrete;

    #[test]
    fn test_unitary_space() {
        let us = UnitarySpace::new(Discrete::new(2));
        let mut rng = thread_rng();

        let mut counts = arr1(&vec![0.0; 2]);
        for _ in 0..5000 {
            let sample = us.sample(&mut rng);
            counts[sample] += 1.0;

            assert!(sample == 0 || sample == 1);
        }

        assert!((counts/5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
        assert_eq!(us.dim(), 1);
        assert_eq!(us.span(), Span::Finite(2));
    }
}
