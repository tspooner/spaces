use crate::prelude::*;

impl<D: Space, const N: usize> Space for [D; N] {
    const DIM: usize = N;

    type Value = [D::Value; N];

    fn card(&self) -> Card { self.iter().fold(Card::Finite(0), |acc, d| acc * d.card()) }

    fn contains(&self, val: &Self::Value) -> bool {
        self.iter()
            .zip(val.iter())
            .all(|(d, x)| d.contains(x))
    }
}

impl<D: Space + Union + Clone, const N: usize> Union for [D; N] {
    fn union(self, other: &Self) -> Self {
        let mut i = 0;

        self.map(|d| {
            i += 1;

            d.union(&other[i-1])
        })
    }
}

impl<D: Space + Intersect + Clone, const N: usize> Intersect for [D; N] {
    fn intersect(self, other: &Self) -> Self {
        let mut i = 0;

        self.map(|d| {
            i += 1;

            d.intersect(&other[i-1])
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Interval;
    use super::*;

    type S = [::std::ops::Range<usize>; 2];

    #[test]
    fn test_dim() {
        assert_eq!(S::DIM, 2);
    }

    #[test]
    fn test_card() {
        assert_eq!([0..2, 0..2].card(), Card::Finite(4));
    }

    #[test]
    fn test_union() {
        let s1 = [Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 3.0)];
        let s2 = [Interval::bounded(-5.0, 0.0), Interval::bounded(1.0, 2.0)];

        assert_eq!(s1.union(&s2), [
            Interval::bounded(-5.0, 5.0),
            Interval::bounded(1.0, 3.0)
        ]);
    }
}
