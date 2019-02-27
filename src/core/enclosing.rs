#![macro_use]
pub trait Enclose<S = Self> {
    /// Return the smallest space enclosing `self` and `other` of type `Self`.
    fn enclose(self, other: &S) -> Self;

    /// Return the smallest space enclosing `self` and all `other_spaces` of type `Self`.
    fn enclose_many(self, other_spaces: &[S]) -> Self where Self: Sized {
        other_spaces.into_iter()
            .fold(self, |acc, other_space| acc.enclose(other_space))
    }
}

macro_rules! impl_auto_enclose {
    ($type:ty, $build:expr) => {
        impl Enclose for $type {
            fn enclose(self, _: &$type) -> Self {
                self
            }
        }
    }
}
