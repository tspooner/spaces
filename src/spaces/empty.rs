use {Space, Span, Surjection};
use rand::ThreadRng;

/// An empty space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct EmptySpace;

impl Space for EmptySpace {
    type Repr = ();

    fn sample(&self, _: &mut ThreadRng) -> Self::Repr {
        ()
    }

    fn dim(&self) -> usize {
        0
    }

    fn span(&self) -> Span {
        Span::Null
    }
}

impl<T> Surjection<T, ()> for EmptySpace {
    fn map(&self, _: T) -> () { () }
}


#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use spaces::{Space, EmptySpace, Span};

    #[test]
    fn test_empty_space() {
        let ns = EmptySpace;
        let mut rng = thread_rng();

        assert_eq!(ns.sample(&mut rng), ());
        assert_eq!(ns.dim(), 0);
        assert_eq!(ns.span(), Span::Null);
    }
}
