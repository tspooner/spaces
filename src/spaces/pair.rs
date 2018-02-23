use super::*;

/// 2-dimensional homogeneous space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PairSpace<D1, D2>((D1, D2))
    where D1: Dimension,
          D2: Dimension;

impl<D1: Dimension, D2: Dimension> PairSpace<D1, D2> {
    pub fn new(d1: D1, d2: D2) -> Self {
        PairSpace((d1, d2))
    }
}

impl PairSpace<dimensions::Continuous, dimensions::Continuous> {
    pub fn partitioned(self, density: usize) -> PairSpace<Partitioned, Partitioned> {
        PairSpace((Partitioned::from_continuous((self.0).0, density),
                   Partitioned::from_continuous((self.0).1, density)))
    }
}

impl<D1: Dimension, D2: Dimension> Space for PairSpace<D1, D2> {
    type Repr = (D1::Value, D2::Value);

    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr {
        ((self.0).0.sample(rng), (self.0).1.sample(rng))
    }

    fn dim(&self) -> usize {
        2
    }

    fn span(&self) -> Span {
        (self.0).0.span()*(self.0).1.span()
    }
}


#[cfg(test)]
mod tests {
    use ndarray::arr1;
    use rand::thread_rng;
    use spaces::{Space, PairSpace, Span};
    use spaces::dimensions::Discrete;

    #[test]
    fn test_pair_space() {
        let ps = PairSpace::new(Discrete::new(2), Discrete::new(2));

        let mut rng = thread_rng();

        let mut c1 = arr1(&vec![0.0; 2]);
        let mut c2 = arr1(&vec![0.0; 2]);
        for _ in 0..5000 {
            let sample = ps.sample(&mut rng);

            c1[sample.0] += 1.0;
            c2[sample.1] += 1.0;

            assert!(sample.0 == 0 || sample.0 == 1);
            assert!(sample.1 == 0 || sample.1 == 1);
        }

        assert!((c1/5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
        assert!((c2/5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));

        assert_eq!(ps.dim(), 2);
        assert_eq!(ps.span(), Span::Finite(4));
    }
}
