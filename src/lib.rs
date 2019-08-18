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

import_all!(core);

pub mod real;
pub mod discrete;

import_all!(empty);
import_all!(interval);
import_all!(partition);

import_all!(pair);
import_all!(n_space);
import_all!(product);

/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of the space.
    type Value: Clone;

    /// Return the dimensionality of the space.
    fn dim(&self) -> Dim;

    /// Return the number of elements in the set comprising the space.
    fn card(&self) -> Card;
}

impl<D: Space> Space for Box<D> {
    type Value = D::Value;

    fn dim(&self) -> Dim { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }
}

impl<'a, D: Space> Space for &'a D {
    type Value = D::Value;

    fn dim(&self) -> Dim { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }
}

/// Trait for defining spaces bounded to lie on an interval I.
///
/// Note: If both `inf` and `sup` are well defined (i.e. are not None), then the interval is
/// bounded and you have defined a compact space; this is true in `spaces` as the Interval type is
/// closed.
pub trait BoundedSpace: Space where Self::Value: PartialOrd {
    /// Returns the value of the dimension's infimum, if it exists.
    fn inf(&self) -> Option<Self::Value>;

    /// Returns the value of the dimension's supremum, if it exists.
    fn sup(&self) -> Option<Self::Value>;

    /// Returns true iff `val` lies within the dimension's bounds (closed).
    fn contains(&self, val: Self::Value) -> bool;

    fn is_left_bounded(&self) -> bool { self.inf().is_some() }

    fn is_right_bounded(&self) -> bool { self.sup().is_some() }

    fn is_compact(&self) -> bool { self.is_left_bounded() && self.is_right_bounded() }
}

/// Trait for defining spaces containing a finite set of values.
pub trait FiniteSpace: BoundedSpace where Self::Value: PartialOrd {
    /// Returns the finite range of values contained by this space.
    fn range(&self) -> ::std::ops::Range<Self::Value>;
}
