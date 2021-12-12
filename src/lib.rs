//! Set/space primitives for defining machine learning problems.
//!
//! `spaces` provides set/space primitives to be used for defining properties of
//! machine learning problems. Traits such as `Space`, and it's derivatives, may
//! be used to define state/action spaces, for example. Mappings between
//! different spaces may also be defined using traits such as `Surjection` to
//! streamline many common preprocessing and type conversion tasks.
extern crate itertools;
extern crate num_traits;

#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde;

mod macros;

import_all!(dim);
import_all!(card);

pub mod real;
pub mod discrete;

import_all!(interval);
import_all!(partition);

import_all!(pair);
import_all!(nd_space);

/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of elements of the space.
    type Value: Clone;

    /// Return the dimensionality of the space.
    fn dim(&self) -> Dim;

    /// Return the number of elements in the set comprising the space.
    fn card(&self) -> Card;

    /// Returns true iff `val` is contained within the space.
    fn contains(&self, val: &Self::Value) -> bool;

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

    fn is_complete(&self) -> bool { self.min().is_some() && self.max().is_some() }
}

impl<D: Space> Space for Box<D> {
    type Value = D::Value;

    fn dim(&self) -> Dim { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }

    fn contains(&self, val: &Self::Value) -> bool { (**self).contains(val) }

    fn min(&self) -> Option<Self::Value> { (**self).min() }

    fn max(&self) -> Option<Self::Value> { (**self).max() }
}

impl<'a, D: Space> Space for &'a D {
    type Value = D::Value;

    fn dim(&self) -> Dim { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }

    fn contains(&self, val: &Self::Value) -> bool { (**self).contains(val) }

    fn min(&self) -> Option<Self::Value> { (**self).min() }

    fn max(&self) -> Option<Self::Value> { (**self).max() }
}

/// Trait for defining spaces containing a finite set of values.
pub trait FiniteSpace: Space where Self::Value: PartialOrd {
    /// Returns the finite range of values contained by this space.
    fn range(&self) -> ::std::ops::Range<Self::Value>;
}

/// Trait for types that implement an idempotent mapping from values of one space onto another.
pub trait Projection<X, Y> {
    /// Map value from domain onto codomain.
    fn project(&self, from: X) -> Y;
}

/// Trait for types that can be combined in the form of a union.
///
/// The union of a collection of sets is the set that contains all elements in the collection.
pub trait Union<S = Self> {
    /// Return the smallest space enclosing `self` and `other` of type `Self`.
    fn union(self, other: &S) -> Self;

    /// Return the smallest space enclosing `self` and all `other_spaces` of type `Self`.
    fn union_many(self, other_spaces: &[S]) -> Self where Self: Sized {
        other_spaces.into_iter().fold(self, |acc, other_space| acc.union(other_space))
    }
}

/// Trait for types that can be combined in the form of an intersection.
///
/// The intersection of a collection of sets is the set that contains only those elements present
/// in each.
pub trait Intersection<S = Self> {
    /// Return the smallest space enclosing `self` and `other` of type `Self`.
    fn intersect(self, other: &S) -> Self;

    /// Return the smallest space enclosing `self` and all `other_spaces` of type `Self`.
    fn intersect_many(self, other_spaces: &[S]) -> Self where Self: Sized {
        other_spaces.into_iter().fold(self, |acc, other_space| acc.intersect(other_space))
    }
}

mod prelude {
    pub use super::{
        Space, FiniteSpace,
        Projection,
        Union, Intersection,
        Dim, Card,
    };
}
