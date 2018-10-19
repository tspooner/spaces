/// A trait for types implementing a mapping from values of one set onto
/// another.
pub trait Surjection<X, Y> {
    /// Map value from domain onto codomain.
    fn map(&self, from: X) -> Y;
}
