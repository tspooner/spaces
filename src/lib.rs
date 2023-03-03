//! Set/space primitives for defining machine learning problems.
//!
//! `spaces` provides set/space primitives to be used for defining properties of
//! machine learning problems. Traits such as `Space`, and it's derivatives, may
//! be used to define state/action spaces, for example.
extern crate itertools;
extern crate num_traits;

pub mod discrete;
pub mod real;

pub extern crate intervals;

use intervals::bounds::OpenOrClosed;

mod arrays;
mod interval;
mod partitions;
mod option;
mod tuples;

///////////////////////////////////////////////////////////////////////////
// Core Definitions
///////////////////////////////////////////////////////////////////////////
/// Trait for types representing spaces (i.e. abstract collections).
pub trait Space {
    /// The data representation for elements of the space.
    type Value;

    /// Return true if the space contains no values.
    ///
    /// ```
    /// # extern crate spaces;
    /// # use spaces::{Space, ops::Intersection, real};
    /// let space = real::reals::<f64>();
    /// assert!(!space.is_empty());
    ///
    /// let space = real::negative_reals::<f64>().intersect(
    ///     real::positive_reals::<f64>()
    /// );
    /// assert!(space.is_none());
    /// ```
    fn is_empty(&self) -> bool;

    /// Returns true iff `val` is contained within the space.
    fn contains(&self, val: &Self::Value) -> bool;
}

/// Trait for types representing ordered spaces.
pub trait OrderedSpace: Space
where Self::Value: PartialOrd
{
    /// Return the infimum of the space, if it exists.
    fn inf(&self) -> Option<OpenOrClosed<Self::Value>>;

    /// Returns the supremum of the space, if it exists.
    fn sup(&self) -> Option<OpenOrClosed<Self::Value>>;

    /// Returns true iff `self` has a well-defined infimum.
    fn is_lower_bounded(&self) -> bool { self.inf().is_some() }

    /// Returns true iff `self` has a well-defined supremum.
    fn is_upper_bounded(&self) -> bool { self.sup().is_some() }

    /// Returns true iff `self` is bounded above and below.
    fn is_bounded(&self) -> bool { self.is_lower_bounded() && self.is_upper_bounded() }
}

/// Trait for defining spaces containing a finite set of values.
pub trait FiniteSpace: Space {
    /// Return the cardinality of the space.
    ///
    /// The cardinality of a space is given by the number of elements
    /// contained within said set.
    fn cardinality(&self) -> usize;
}

/// Trait for `Space` types that have an associated value iterator.
pub trait IterableSpace: Space {
    /// The associated iterator type.
    type ElemIter: Iterator<Item = Self::Value>;

    /// Return an iterator over the elements of this space.
    fn elements(&self) -> Self::ElemIter;
}

///////////////////////////////////////////////////////////////////////////
// Set Operations
///////////////////////////////////////////////////////////////////////////
pub mod ops;

///////////////////////////////////////////////////////////////////////////
// Prelude
///////////////////////////////////////////////////////////////////////////
mod prelude {
    pub use super::{
        ops::{Union, Intersection, Closure},
        FiniteSpace, OrderedSpace, Space, IterableSpace,
    };
}
