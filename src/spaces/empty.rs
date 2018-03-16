use {Space, Span, Surjection};
use rand::ThreadRng;

/// An empty space.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct EmptySpace;

impl Space for EmptySpace {
    type Repr = ();

    fn dim(&self) -> usize {
        0
    }

    fn span(&self) -> Span {
        Span::Null
    }

    fn sample(&self, _: &mut ThreadRng) -> Self::Repr {
        ()
    }
}

impl<T> Surjection<T, ()> for EmptySpace {
    fn map(&self, _: T) -> () { () }
}


#[cfg(test)]
mod tests {
    use {Space, Span, Surjection, EmptySpace};
    use rand::thread_rng;

    #[test]
    fn test_dim() {
        assert_eq!(EmptySpace.dim(), 0);
    }

    #[test]
    fn test_span() {
        assert_eq!(EmptySpace.span(), Span::Null);
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
}
