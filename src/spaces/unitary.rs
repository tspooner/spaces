use {Dimension, Space, Span, Surjection};
use dimensions::{Continuous, Partitioned};
use rand::ThreadRng;

/// 1-dimensional space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct UnitarySpace<D: Dimension>(D);

impl<D: Dimension> UnitarySpace<D> {
    pub fn new(d: D) -> Self {
        UnitarySpace(d)
    }
}

impl UnitarySpace<Continuous> {
    pub fn partitioned(self, density: usize) -> UnitarySpace<Partitioned> {
        UnitarySpace(Partitioned::from_continuous(self.0, density))
    }
}

impl<D: Dimension> Space for UnitarySpace<D> {
    type Repr = D::Value;

    fn dim(&self) -> usize {
        1
    }

    fn span(&self) -> Span {
        self.0.span()
    }

    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr {
        self.0.sample(rng)
    }
}

impl<D, X> Surjection<X, D::Value> for UnitarySpace<D>
where
    D: Dimension + Surjection<X, <D as Dimension>::Value>,
{
    fn map(&self, val: X) -> D::Value {
        self.0.map(val)
    }
}


#[cfg(test)]
mod tests {
    use {Space, UnitarySpace, Span, Surjection};
    use dimensions::{Continuous, Discrete, Partitioned};
    use ndarray::arr1;
    use rand::thread_rng;

    #[test]
    fn test_dim() {
        assert_eq!(UnitarySpace::new(Discrete::new(2)).dim(), 1);
    }

    #[test]
    fn test_span() {
        assert_eq!(UnitarySpace::new(Discrete::new(2)).span(), Span::Finite(2));
    }

    #[test]
    fn test_sample() {
        let us = UnitarySpace::new(Discrete::new(2));
        let mut rng = thread_rng();

        let mut counts = arr1(&vec![0.0; 2]);
        for _ in 0..5000 {
            let sample = us.sample(&mut rng);
            counts[sample] += 1.0;

            assert!(sample == 0 || sample == 1);
        }

        assert!((counts/5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
    }

    #[test]
    fn test_partitioned() {
        let us = UnitarySpace::new(Continuous::new(0.0, 5.0));
        let us = us.partitioned(5);

        assert_eq!(us.0, Partitioned::new(0.0, 5.0, 5));
    }

    #[test]
    fn test_surjection() {
        let us = UnitarySpace::new(Continuous::new(0.0, 5.0));

        assert_eq!(us.map(6.0), 5.0);
        assert_eq!(us.map(2.5), 2.5);
        assert_eq!(us.map(-1.0), 0.0);
    }
}
