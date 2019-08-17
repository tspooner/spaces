#![macro_use]
pub trait Union<S = Self> {
    /// Return the smallest space enclosing `self` and `other` of type `Self`.
    fn union(self, other: &S) -> Self;

    /// Return the smallest space enclosing `self` and all `other_spaces` of type `Self`.
    fn union_many(self, other_spaces: &[S]) -> Self where Self: Sized {
        other_spaces.into_iter().fold(self, |acc, other_space| acc.union(other_space))
    }
}

macro_rules! impl_auto_union {
    ($type:ty, $build:expr) => {
        impl Union for $type {
            fn union(self, _: &$type) -> Self {
                self
            }
        }
    }
}
