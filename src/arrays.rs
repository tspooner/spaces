use crate::prelude::*;

impl<D: Space, const N: usize> Space for [D; N] {
    type Value = [D::Value; N];

    fn is_empty(&self) -> bool { self.iter().any(|d| d.is_empty()) }

    fn contains(&self, val: &Self::Value) -> bool {
        self.iter().zip(val.iter()).all(|(d, x)| d.contains(x))
    }
}

impl<D: FiniteSpace, const N: usize> FiniteSpace for [D; N] {
    fn cardinality(&self) -> usize { self.iter().map(|d| d.cardinality()).product() }
}

// impl<D: Space + Union + Clone, const N: usize> Union for [D; N] {
// fn union(self, other: &Self) -> Self {
// let mut i = 0;

// self.map(|d| {
// i += 1;

// d.union(&other[i-1])
// })
// }
// }

// impl<D: Space + Intersect + Clone, const N: usize> Intersect for [D; N] {
// fn intersect(self, other: &Self) -> Self {
// let mut i = 0;

// self.map(|d| {
// i += 1;

// d.intersect(&other[i-1])
// })
// }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intervals::Interval;

    #[test]
    fn test_card() {
        let a = Interval::lorc(0usize, 2usize);
        let b = a.clone();

        assert_eq!([a, b].cardinality(), 4);
    }

    // #[test]
    // fn test_union() {
    // let s1 = [real::reals(); 2];
    // let s2 = [real::reals(); 2];

    // assert_eq!(s1.union(&s2), [real::reals(); 2]);
    // }
}
