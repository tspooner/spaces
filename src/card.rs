use std::ops::Mul;

/// Measure of the cardinality (#) of a set.
///
/// `Card` is used to represent the number of possible values that are contained by a space. Take,
/// for example, a 2-dimensional space, each with a finite set of values. In this case we have the
/// following:
/// ```
/// use spaces::{Space, Card};
///
/// let d1 = 0..5;
/// let d2 = 0..10;
/// let space = (d1, d2);
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
/// assert_eq!(s1 * s2, Card::Finite(50));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Card {
    Finite(usize),
    Infinite,
}

impl Card {
    pub fn is_zero(&self) -> bool {
        match self {
            &Card::Finite(n) => n == 0,
            _ => false,
        }
    }

    pub fn is_finite(&self) -> bool {
        match self {
            &Card::Finite(_) => true,
            _ => false,
        }
    }

    pub fn is_infinite(&self) -> bool {
        match self {
            &Card::Infinite => true,
            _ => false,
        }
    }

    pub fn expect_finite(&self, msg: &str) -> usize {
        match self {
            &Card::Finite(n) => n,
            _ => panic!("{}", msg),
        }
    }
}

impl Mul for Card {
    type Output = Card;

    fn mul(self, rhs: Card) -> Card {
        match (self, rhs) {
            (Card::Infinite, _) | (_, Card::Infinite) => Card::Infinite,
            (Card::Finite(0), Card::Finite(a)) | (Card::Finite(a), Card::Finite(0)) =>
                Card::Finite(a),
            (Card::Finite(ls), Card::Finite(rs)) => Card::Finite(ls * rs),
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
        assert_eq!(Card::Infinite, Card::Infinite);

        assert_eq!(Card::Finite(0), Card::Finite(0));
        assert_eq!(Card::Finite(1), Card::Finite(1));
        assert_eq!(Card::Finite(5), Card::Finite(5));
        assert_eq!(Card::Finite(10), Card::Finite(10));
    }

    #[test]
    fn test_inequality() {
        assert_ne!(Card::Finite(0), Card::Infinite);

        assert_ne!(Card::Infinite, Card::Finite(0));
        assert_ne!(Card::Infinite, Card::Finite(1));

        assert_ne!(Card::Finite(0), Card::Finite(1));
        assert_ne!(Card::Finite(1), Card::Finite(0));
        assert_ne!(Card::Finite(1), Card::Infinite);
        assert_ne!(Card::Finite(1), Card::Finite(10));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Card::Finite(0), Card::Finite(0));
        assert_eq!(Card::Infinite * Card::Infinite, Card::Infinite);

        assert_eq!(Card::Finite(0) * Card::Infinite, Card::Infinite);
        assert_eq!(Card::Infinite * Card::Finite(0), Card::Infinite);

        assert_eq!(Card::Finite(1) * Card::Infinite, Card::Infinite);
        assert_eq!(Card::Finite(5) * Card::Infinite, Card::Infinite);

        assert_eq!(Card::Finite(1) * Card::Finite(1), Card::Finite(1));
        assert_eq!(Card::Finite(1) * Card::Finite(5), Card::Finite(5));
        assert_eq!(Card::Finite(5) * Card::Finite(1), Card::Finite(5));
        assert_eq!(Card::Finite(5) * Card::Finite(5), Card::Finite(25));
    }

    #[test]
    #[should_panic]
    fn test_into_infinite() {
        let s = Card::Infinite;
        let _: usize = s.into();
    }

    #[test]
    fn test_into_finite() {
        for i in vec![0, 1, 5, 10] {
            let d = Card::Finite(i);
            let v: usize = d.into();

            assert_eq!(v, i);
        }
    }
}
