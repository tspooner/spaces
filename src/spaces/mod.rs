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

/// Geometric space for agent actions.
///
/// Currently the only supported representation for actions. In future we will need to handle
/// continuous actions.
pub type ActionSpace = UnitarySpace<dimensions::Discrete>;


pub mod empty;
pub use empty::EmptySpace;

pub mod unitary;
pub use unitary::UnitarySpace;

pub mod pair;
pub use pair::PairSpace;

pub mod regular;
pub use regular::RegularSpace;

pub mod named;
pub use named::NamedSpace;
