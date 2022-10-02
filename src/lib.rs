//! Set/space primitives for defining machine learning problems.
//!
//! `spaces` provides set/space primitives to be used for defining properties of
//! machine learning problems. Traits such as `Space`, and it's derivatives, may
//! be used to define state/action spaces, for example. Mappings between
//! different spaces may also be defined using traits such as `Surjection` to
//! streamline many common preprocessing and type conversion tasks.
extern crate array_init;
extern crate itertools;
extern crate num_traits;

pub mod discrete;
pub mod real;

pub extern crate intervals;

mod arrays;
mod interval_impls;
mod option;
mod tuples;

///////////////////////////////////////////////////////////////////////////
// Core Definitions
///////////////////////////////////////////////////////////////////////////
/// Trait for types representing geometric spaces.
pub trait Space {
    /// The data representation for elements of the space.
    type Value;

    /// Return true iff the space contains no values.
    ///
    /// ```
    /// # extern crate spaces;
    /// # use spaces::{Space, ops::Intersection, real};
    /// let space = real::reals::<f64>();
    /// assert!(!space.is_empty());
    ///
    /// let space = real::negative_reals::<f64>().intersection(
    ///     real::positive_reals::<f64>()
    /// );
    /// assert!(space.is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Returns true iff `val` is contained within the space.
    fn contains(&self, val: &Self::Value) -> bool;
}

/// Trait for types representing ordered spaces.
pub trait OrderedSpace: Space
where Self::Value: PartialOrd
{
    /// Returns the value of the space's minimum value, if it exists.
    fn min(&self) -> Option<Self::Value> { None }

    /// Return the infimum of the space.
    fn inf(&self) -> Option<Self::Value> { self.min() }

    /// Returns the value of the dimension's supremum, if it exists.
    fn max(&self) -> Option<Self::Value> { None }

    /// Returns the supremum of the space.
    fn sup(&self) -> Option<Self::Value> { self.max() }

    /// Returns true iff `self` has a well-defined infimum.
    fn is_lower_bounded(&self) -> bool { self.inf().is_some() }

    /// Returns true iff `self` has a well-defined supremum.
    fn is_upper_bounded(&self) -> bool { self.sup().is_some() }

    /// Returns true iff `self` has a well-defined minimum and maximum.
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

///////////////////////////////////////////////////////////////////////////
// Set Operations
///////////////////////////////////////////////////////////////////////////
pub mod ops;

///////////////////////////////////////////////////////////////////////////
// Prelude
///////////////////////////////////////////////////////////////////////////
mod prelude {
    pub use super::{ops, FiniteSpace, OrderedSpace, Space};
}
