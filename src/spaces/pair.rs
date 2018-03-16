use {Dimension, Space, Span, Surjection};
use dimensions::{Continuous, Partitioned};
use rand::ThreadRng;

/// 2-dimensional homogeneous space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PairSpace<D1, D2>(pub D1, pub D2)
    where D1: Dimension,
          D2: Dimension;

impl<D1: Dimension, D2: Dimension> PairSpace<D1, D2> {
    pub fn new(d1: D1, d2: D2) -> Self {
        PairSpace(d1, d2)
    }
}

impl PairSpace<Continuous, Continuous> {
    pub fn partitioned(self, density: usize) -> PairSpace<Partitioned, Partitioned> {
        PairSpace(Partitioned::from_continuous(self.0, density),
                  Partitioned::from_continuous(self.1, density))
    }
}

impl<D1: Dimension, D2: Dimension> Space for PairSpace<D1, D2> {
    type Repr = (D1::Value, D2::Value);

    fn dim(&self) -> usize {
        2
    }

    fn span(&self) -> Span {
        self.0.span()*self.1.span()
    }

    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr {
        (self.0.sample(rng), self.1.sample(rng))
    }
}

impl<D1, X1, D2, X2> Surjection<(X1, X2), (D1::Value, D2::Value)> for PairSpace<D1, D2>
where
    D1: Dimension + Surjection<X1, <D1 as Dimension>::Value>,
    D2: Dimension + Surjection<X2, <D2 as Dimension>::Value>,
{
    fn map(&self, val: (X1, X2)) -> (D1::Value, D2::Value) {
        (self.0.map(val.0), self.1.map(val.1))
    }
}


#[cfg(test)]
mod tests {
    use {Space, PairSpace, Span, Surjection};
    use dimensions::{Continuous, Discrete, Partitioned};
    use ndarray::arr1;
    use rand::thread_rng;

    #[test]
    fn test_dim() {
        assert_eq!(PairSpace::new(Discrete::new(2), Discrete::new(2)).dim(), 2);
    }

    #[test]
    fn test_span() {
        assert_eq!(PairSpace::new(Discrete::new(2), Discrete::new(2)).span(), Span::Finite(4));
    }

    #[test]
    fn test_sample() {
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
    }

    #[test]
    fn test_partitioned() {
        let ps = PairSpace::new(Continuous::new(0.0, 5.0), Continuous::new(1.0, 2.0));
        let ps = ps.partitioned(5);

        assert_eq!(ps.0, Partitioned::new(0.0, 5.0, 5));
        assert_eq!(ps.1, Partitioned::new(1.0, 2.0, 5));
    }

    #[test]
    fn test_surjection() {
        let ps = PairSpace::new(Continuous::new(0.0, 5.0), Continuous::new(1.0, 2.0));

        assert_eq!(ps.map((6.0, 0.0)), (5.0, 1.0));
        assert_eq!(ps.map((2.5, 1.5)), (2.5, 1.5));
        assert_eq!(ps.map((-1.0, 10.0)), (0.0, 2.0));
    }
}
