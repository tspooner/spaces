use continuous::Interval;
use core::{Space, Card, Surjection};
use discrete::Partition;
use rand::Rng;

/// 2-dimensional homogeneous space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PairSpace<D1, D2>(pub D1, pub D2)
where
    D1: Space,
    D2: Space;

impl<D1: Space, D2: Space> PairSpace<D1, D2> {
    pub fn new(d1: D1, d2: D2) -> Self { PairSpace(d1, d2) }
}

impl PairSpace<Interval, Interval> {
    pub fn partitioned(self, density: usize) -> PairSpace<Partition, Partition> {
        PairSpace(
            Partition::from_continuous(self.0, density),
            Partition::from_continuous(self.1, density),
        )
    }
}

impl<D1: Space, D2: Space> Space for PairSpace<D1, D2> {
    type Value = (D1::Value, D2::Value);

    fn dim(&self) -> usize { 2 }

    fn card(&self) -> Card { self.0.card() * self.1.card() }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (D1::Value, D2::Value) {
        (self.0.sample(rng), self.1.sample(rng))
    }
}

impl<D1, X1, D2, X2> Surjection<(X1, X2), (D1::Value, D2::Value)> for PairSpace<D1, D2>
where
    D1: Space + Surjection<X1, <D1 as Space>::Value>,
    D2: Space + Surjection<X2, <D2 as Space>::Value>,
{
    fn map(&self, val: (X1, X2)) -> (D1::Value, D2::Value) {
        (self.0.map(val.0), self.1.map(val.1))
    }
}

#[cfg(test)]
mod tests {
    extern crate ndarray;

    use continuous::Interval;
    use core::{Space, Card, Surjection};
    use discrete::{Discrete, Partition};
    use product::PairSpace;
    use rand::thread_rng;
    use self::ndarray::arr1;

    #[test]
    fn test_dim() {
        assert_eq!(PairSpace::new(Discrete::new(2), Discrete::new(2)).dim(), 2);
    }

    #[test]
    fn test_card() {
        assert_eq!(
            PairSpace::new(Discrete::new(2), Discrete::new(2)).card(),
            Card::Finite(4)
        );
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

        assert!((c1 / 5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
        assert!((c2 / 5000.0).all_close(&arr1(&vec![0.5; 2]), 1e-1));
    }

    #[test]
    fn test_partitioned() {
        let ps = PairSpace::new(Interval::closed(0.0, 5.0), Interval::closed(1.0, 2.0));
        let ps = ps.partitioned(5);

        assert_eq!(ps.0, Partition::new(0.0, 5.0, 5));
        assert_eq!(ps.1, Partition::new(1.0, 2.0, 5));
    }

    #[test]
    fn test_surjection() {
        let ps = PairSpace::new(Interval::closed(0.0, 5.0), Interval::closed(1.0, 2.0));

        assert_eq!(ps.map((6.0, 0.0)), (5.0, 1.0));
        assert_eq!(ps.map((2.5, 1.5)), (2.5, 1.5));
        assert_eq!(ps.map((-1.0, 10.0)), (0.0, 2.0));
    }
}
