use crate::{Space, Card, Dim, Surjection};
use std::fmt;

/// A space filled with... nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Empty;

impl Space for Empty {
    type Value = ();

    fn dim(&self) -> Dim { Dim::Finite(0) }

    fn card(&self) -> Card { Card::Finite(0) }
}

impl<T> Surjection<T, ()> for Empty {
    fn map_onto(&self, _: T) -> () { () }
}

impl fmt::Display for Empty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{2205}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serialize")]
    extern crate serde_test;
    #[cfg(feature = "serialize")]
    use self::serde_test::{assert_tokens, Token};

    #[test]
    fn test_card() {
        let d = Empty;

        assert_eq!(d.card(), Card::Finite(0));
    }

    #[test]
    fn test_surjection() {
        let d = Empty;

        for i in 0..10 {
            assert_eq!(d.map_onto(i), ());
            assert_eq!(d.map_onto(i), ());
        }
    }

    #[cfg(feature = "serialize")]
    #[test]
    fn test_serialisation() {
        let d = Empty;

        assert_tokens(&d, &[Token::UnitStruct { name: "Empty" }]);
    }
}
