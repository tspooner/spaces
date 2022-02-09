use crate::prelude::*;

impl<D: Space, const N: usize> Space for [D; N] {
    type Value = [D::Value; N];

    fn dim(&self) -> Dim { self.iter().fold(Dim::Finite(0), |acc, d| acc + d.dim()) }

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

impl<D, X, const N: usize> Project<[X; N], [D::Value; N]> for [D; N]
where
    D: Space + Project<X, <D as Space>::Value>,
    D::Value: Default,
{
    fn project(&self, val: [X; N]) -> [D::Value; N] {
        let mut i = 0;

        val.map(|x| {
            i += 1;

            self[i-1].project(x)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Interval;
    use super::*;

    #[test]
    fn test_dim() {
        assert_eq!([0..2, 0..2].dim(), Dim::Finite(2));
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

    #[test]
    fn test_project() {
        let space = [Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0)];

        assert_eq!(space.project([6.0, 0.0]), [5.0, 1.0]);
        assert_eq!(space.project([2.5, 1.5]), [2.5, 1.5]);
        assert_eq!(space.project([-1.0, 3.0]), [0.0, 2.0]);
    }
}
