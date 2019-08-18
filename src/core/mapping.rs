/// Trait for types that implement a mapping from values of one set onto another.
pub trait Surjection<X, Y> {
    /// Map value from domain onto codomain.
    fn map_onto(&self, from: X) -> Y;
}
