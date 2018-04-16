use {Space, Card, Surjection};

use rand::ThreadRng;

/// An empty space.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmptySpace;

impl Space for EmptySpace {
    type Value = ();

    fn dim(&self) -> usize { 0 }

    fn card(&self) -> Card { Card::Null }

    fn sample(&self, _: &mut ThreadRng) -> () { () }
}

impl<T> Surjection<T, ()> for EmptySpace {
    fn map(&self, _: T) -> () { () }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use rand::thread_rng;
    use {EmptySpace, Space, Card, Surjection};

    #[test]
    fn test_copy() {
        let s = EmptySpace;

        assert_eq!(s, s);
    }

    #[test]
    fn test_dim() {
        assert_eq!(EmptySpace.dim(), 0);
    }

    #[test]
    fn test_card() {
        assert_eq!(EmptySpace.card(), Card::Null);
    }

    #[test]
    fn test_sample() {
        let mut rng = thread_rng();

        assert_eq!(EmptySpace.sample(&mut rng), ());
    }

    #[test]
    fn test_surjection() {
        assert_eq!(EmptySpace.map(1), ());
        assert_eq!(EmptySpace.map(1.0), ());
        assert_eq!(EmptySpace.map("test"), ());
        assert_eq!(EmptySpace.map(Some(true)), ());
    }

    #[test]
    fn test_serialisation() {
        let d = EmptySpace;

        assert_tokens(&d, &[Token::UnitStruct { name: "EmptySpace" }]);
    }
}
