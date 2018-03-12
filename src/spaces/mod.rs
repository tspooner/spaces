use Span;
use dimensions::{self, Dimension, Partitioned};
use rand::ThreadRng;
use std::fmt::Debug;
use std::ops::{Add, Index};
use std::iter::FromIterator;
use std::slice::Iter as SliceIter;
use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;


/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of the space.
    type Repr: Debug + Clone;

    /// Generate a random sample from the space.
    fn sample(&self, rng: &mut ThreadRng) -> Self::Repr;

    /// Return the number of dimensions in the space.
    fn dim(&self) -> usize;

    /// Return the number of linear combinations of values in the space.
    fn span(&self) -> Span;
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
