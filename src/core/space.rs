use core::Card;
use std::fmt::Debug;
use std::ops::Range;

/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of the space.
    type Value: Debug + Clone;

    /// Return the dimensionality of the space.
    fn dim(&self) -> usize;

    /// Return the number of elements in the set composing the space.
    fn card(&self) -> Card;
}

impl<D: Space> Space for Box<D> {
    type Value = D::Value;

    fn dim(&self) -> usize { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }
}

impl<'a, D: Space> Space for &'a D {
    type Value = D::Value;

    fn dim(&self) -> usize { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }
}

/// Trait for defining spaces bounded to lie on an interval I.
///
/// Note: If both `inf` and `sup` are well defined (i.e. are not None), then the interval is
/// bounded and you have defined a compact space; this is true in `spaces` as the Interval type is
/// closed.
pub trait BoundedSpace: Space
where Self::Value: PartialOrd
{
    /// The upper/lower bound type; not necessarily equal to `Space::Value`.
    type BoundValue: PartialOrd + Copy;

    /// Returns the value of the dimension's infimum, if it exists.
    fn inf(&self) -> Option<Self::BoundValue>;

    /// Returns the value of the dimension's supremum, if it exists.
    fn sup(&self) -> Option<Self::BoundValue>;

    /// Returns true iff `val` lies within the dimension's bounds (closed).
    fn contains(&self, val: Self::BoundValue) -> bool;
}

/// Trait for defining spaces containing a finite set of values.
pub trait FiniteSpace: BoundedSpace
where Self::Value: PartialOrd
{
    /// Returns the finite range of values contained by this space.
    fn range(&self) -> Range<Self::Value>;
}
