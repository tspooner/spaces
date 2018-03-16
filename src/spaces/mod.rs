use Span;
use dimensions::{self, Dimension, Partitioned};
use rand::ThreadRng;
use std::fmt::Debug;


/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of the space.
    type Repr: Debug + Clone;

    /// Return the number of dimensions in the space.
    fn dim(&self) -> usize;

    /// Return the number of linear combinations of values in the space.
    fn span(&self) -> Span;

    /// Generate a random sample from the space.
    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr;
}


mod empty;
pub use self::empty::EmptySpace;

mod unitary;
pub use self::unitary::UnitarySpace;

mod pair;
pub use self::pair::PairSpace;

mod regular;
pub use self::regular::RegularSpace;

mod named;
pub use self::named::NamedSpace;
