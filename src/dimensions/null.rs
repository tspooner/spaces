use {Space, Surjection, Span};

use rand::ThreadRng;

/// A null dimension.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Null;

impl Space for Null {
    type Value = ();

    fn dim(&self) -> usize { 1 }

    fn span(&self) -> Span { Span::Null }

    fn sample(&self, _: &mut ThreadRng) -> () { () }
}

impl<T> Surjection<T, ()> for Null {
    fn map(&self, _: T) -> () { () }
}


#[cfg(test)]
mod tests {
    extern crate serde_test;

    use rand::{thread_rng, Rng};
    use self::serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_span() {
        let d = Null;

        assert_eq!(d.span(), Span::Null);
    }

    #[test]
    fn test_sampling() {
        let d = Null;
        let mut rng = thread_rng();

        for _ in 0..10 {
            assert_eq!(d.sample(&mut rng), ());
        }
    }

    #[test]
    fn test_surjection() {
        let d = Null;
        let mut rng = thread_rng();

        for _ in 0..10 {
            assert_eq!(d.map(rng.next_f64()), ());
            assert_eq!(d.map(rng.next_u64()), ());
        }
    }

    #[test]
    fn test_serialisation() {
        let d = Null;

        assert_tokens(&d, &[Token::UnitStruct { name: "Null" }]);
    }
}
