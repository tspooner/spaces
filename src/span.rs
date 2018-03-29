use std::ops::Mul;


/// Measure of the span of a vector space.
///
/// `Span` is typically used to compute the number of possible values that can be reached within
/// some vector space. For example, for a space with 2 dimensions, each with a finite set of
/// values, we have:
///
/// ```
/// use spaces::{PairSpace, Space, Span};
/// use spaces::dimensions::Discrete;
///
/// let d1 = Discrete::new(5);
/// let d2 = Discrete::new(10);
/// let space = PairSpace::new(d1, d2);
///
/// assert_eq!(space.span(), Span::Finite(50));
/// ```
///
/// Internally, this above code does the following:
///
/// ```
/// use spaces::Span;
///
/// let s1 = Span::Finite(5);
/// let s2 = Span::Finite(10);
///
/// assert_eq!(s1*s2, Span::Finite(50));
/// ```
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Span {
    Null,
    Finite(usize),
    Infinite,
}

impl Mul for Span {
    type Output = Span;

    fn mul(self, rhs: Span) -> Span {
        match self {
            Span::Null => rhs,
            Span::Infinite => self,
            Span::Finite(ls) => {
                match rhs {
                    Span::Null => self,
                    Span::Infinite => rhs,
                    Span::Finite(rs) => Span::Finite(ls * rs),
                }
            }
        }
    }
}

impl Into<usize> for Span {
    fn into(self) -> usize {
        match self {
            Span::Finite(e) => e,
            _ => panic!("Span type has no integer representation."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn test_equality() {
        assert_eq!(Span::Null, Span::Null);
        assert_eq!(Span::Infinite, Span::Infinite);

        assert_eq!(Span::Finite(1), Span::Finite(1));
        assert_eq!(Span::Finite(5), Span::Finite(5));
        assert_eq!(Span::Finite(10), Span::Finite(10));
    }

    #[test]
    fn test_inequality() {
        assert_ne!(Span::Null, Span::Infinite);
        assert_ne!(Span::Null, Span::Finite(1));

        assert_ne!(Span::Infinite, Span::Null);
        assert_ne!(Span::Infinite, Span::Finite(1));

        assert_ne!(Span::Finite(1), Span::Null);
        assert_ne!(Span::Finite(1), Span::Infinite);
        assert_ne!(Span::Finite(1), Span::Finite(10));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Span::Null * Span::Null, Span::Null);
        assert_eq!(Span::Infinite * Span::Infinite, Span::Infinite);

        assert_eq!(Span::Null * Span::Infinite, Span::Infinite);
        assert_eq!(Span::Infinite * Span::Null, Span::Infinite);

        assert_eq!(Span::Finite(1) * Span::Infinite, Span::Infinite);
        assert_eq!(Span::Finite(5) * Span::Infinite, Span::Infinite);

        assert_eq!(Span::Finite(1) * Span::Finite(1), Span::Finite(1));
        assert_eq!(Span::Finite(1) * Span::Finite(5), Span::Finite(5));
        assert_eq!(Span::Finite(5) * Span::Finite(1), Span::Finite(5));
        assert_eq!(Span::Finite(5) * Span::Finite(5), Span::Finite(25));
    }

    #[test]
    #[should_panic]
    fn test_into_null() {
        let s = Span::Null;
        let _: usize = s.into();
    }

    #[test]
    #[should_panic]
    fn test_into_infinite() {
        let s = Span::Infinite;
        let _: usize = s.into();
    }

    #[test]
    fn test_into_finite() {
        for i in vec![1, 5, 10] {
            let d = Span::Finite(i);
            let v: usize = d.into();

            assert_eq!(v, i);
        }
    }
}
