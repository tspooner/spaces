use std::ops::Mul;

/// Measure of the cardinality (#) of a set.
///
/// `Card` is used to represent the number of possible values that are contained by a space. Take,
/// for example, a 2-dimensional space, each with a finite set of values. In this case we have the
/// following:
/// ```
/// use spaces::{Space, Card};
/// use spaces::product::PairSpace;
/// use spaces::discrete::Discrete;
///
/// let d1 = Discrete::new(5);
/// let d2 = Discrete::new(10);
/// let space = PairSpace::new(d1, d2);
///
/// assert_eq!(space.card(), Card::Finite(50));
/// ```
///
/// Internally, this above code does the following:
///
/// ```
/// use spaces::Card;
///
/// let s1 = Card::Finite(5);
/// let s2 = Card::Finite(10);
///
/// assert_eq!(s1*s2, Card::Finite(50));
/// ```
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Card {
    Null,
    Finite(usize),
    Infinite,
}

impl Mul for Card {
    type Output = Card;

    fn mul(self, rhs: Card) -> Card {
        match self {
            Card::Null => rhs,
            Card::Infinite => self,
            Card::Finite(ls) => match rhs {
                Card::Null => self,
                Card::Infinite => rhs,
                Card::Finite(rs) => Card::Finite(ls * rs),
            },
        }
    }
}

impl Into<usize> for Card {
    fn into(self) -> usize {
        match self {
            Card::Finite(e) => e,
            _ => panic!("Card type has no integer representation."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn test_equality() {
        assert_eq!(Card::Null, Card::Null);
        assert_eq!(Card::Infinite, Card::Infinite);

        assert_eq!(Card::Finite(1), Card::Finite(1));
        assert_eq!(Card::Finite(5), Card::Finite(5));
        assert_eq!(Card::Finite(10), Card::Finite(10));
    }

    #[test]
    fn test_inequality() {
        assert_ne!(Card::Null, Card::Infinite);
        assert_ne!(Card::Null, Card::Finite(1));

        assert_ne!(Card::Infinite, Card::Null);
        assert_ne!(Card::Infinite, Card::Finite(1));

        assert_ne!(Card::Finite(1), Card::Null);
        assert_ne!(Card::Finite(1), Card::Infinite);
        assert_ne!(Card::Finite(1), Card::Finite(10));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Card::Null * Card::Null, Card::Null);
        assert_eq!(Card::Infinite * Card::Infinite, Card::Infinite);

        assert_eq!(Card::Null * Card::Infinite, Card::Infinite);
        assert_eq!(Card::Infinite * Card::Null, Card::Infinite);

        assert_eq!(Card::Finite(1) * Card::Infinite, Card::Infinite);
        assert_eq!(Card::Finite(5) * Card::Infinite, Card::Infinite);

        assert_eq!(Card::Finite(1) * Card::Finite(1), Card::Finite(1));
        assert_eq!(Card::Finite(1) * Card::Finite(5), Card::Finite(5));
        assert_eq!(Card::Finite(5) * Card::Finite(1), Card::Finite(5));
        assert_eq!(Card::Finite(5) * Card::Finite(5), Card::Finite(25));
    }

    #[test]
    #[should_panic]
    fn test_into_null() {
        let s = Card::Null;
        let _: usize = s.into();
    }

    #[test]
    #[should_panic]
    fn test_into_infinite() {
        let s = Card::Infinite;
        let _: usize = s.into();
    }

    #[test]
    fn test_into_finite() {
        for i in vec![1, 5, 10] {
            let d = Card::Finite(i);
            let v: usize = d.into();

            assert_eq!(v, i);
        }
    }
}
