use std::ops::Add;

/// Measure of the dimensionality of the elements of a set.
/// ```
/// use spaces::{Space, Dim};
///
/// let d1 = 0..5;
/// let d2 = 0..10;
/// let space = (d1, d2);
///
/// assert_eq!(space.dim(), Dim::Finite(2));
/// ```
///
/// Internally, this above code does the following:
///
/// ```
/// use spaces::Dim;
///
/// let s1 = Dim::one();
/// let s2 = Dim::one();
///
/// assert_eq!(s1 + s2, Dim::Finite(2));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Dim {
    Finite(usize),
    Infinite,
}

impl Dim {
    pub fn one() -> Dim { Dim::Finite(1) }
}

impl Add for Dim {
    type Output = Dim;

    fn add(self, rhs: Dim) -> Dim {
        match (self, rhs) {
            (Dim::Infinite, _) | (_, Dim::Infinite) => Dim::Infinite,
            (Dim::Finite(ls), Dim::Finite(rs)) => Dim::Finite(ls + rs),
        }
    }
}

impl Into<usize> for Dim {
    fn into(self) -> usize {
        match self {
            Dim::Finite(e) => e,
            _ => panic!("Dim type has no integer representation."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dim;

    #[test]
    fn test_equality() {
        assert_eq!(Dim::Infinite, Dim::Infinite);

        assert_eq!(Dim::Finite(0), Dim::Finite(0));
        assert_eq!(Dim::Finite(1), Dim::Finite(1));
        assert_eq!(Dim::Finite(5), Dim::Finite(5));
        assert_eq!(Dim::Finite(10), Dim::Finite(10));
    }

    #[test]
    fn test_inequality() {
        assert_ne!(Dim::Finite(0), Dim::Infinite);

        assert_ne!(Dim::Infinite, Dim::Finite(0));
        assert_ne!(Dim::Infinite, Dim::Finite(1));

        assert_ne!(Dim::Finite(0), Dim::Finite(1));
        assert_ne!(Dim::Finite(1), Dim::Finite(0));
        assert_ne!(Dim::Finite(1), Dim::Infinite);
        assert_ne!(Dim::Finite(1), Dim::Finite(10));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Dim::Finite(0), Dim::Finite(0));
        assert_eq!(Dim::Infinite + Dim::Infinite, Dim::Infinite);

        assert_eq!(Dim::Finite(0) + Dim::Infinite, Dim::Infinite);
        assert_eq!(Dim::Infinite + Dim::Finite(0), Dim::Infinite);

        assert_eq!(Dim::Finite(1) + Dim::Infinite, Dim::Infinite);
        assert_eq!(Dim::Finite(5) + Dim::Infinite, Dim::Infinite);

        assert_eq!(Dim::Finite(1) + Dim::Finite(1), Dim::Finite(2));
        assert_eq!(Dim::Finite(1) + Dim::Finite(5), Dim::Finite(6));
        assert_eq!(Dim::Finite(5) + Dim::Finite(1), Dim::Finite(6));
        assert_eq!(Dim::Finite(5) + Dim::Finite(5), Dim::Finite(10));
    }

    #[test]
    #[should_panic]
    fn test_into_infinite() {
        let s = Dim::Infinite;
        let _: usize = s.into();
    }

    #[test]
    fn test_into_finite() {
        for i in vec![0, 1, 5, 10] {
            let d = Dim::Finite(i);
            let v: usize = d.into();

            assert_eq!(v, i);
        }
    }
}
