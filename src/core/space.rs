use core::Card;
use rand::Rng;
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

    /// Generate a random sample from the space.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::Value;
}

impl<D: Space> Space for Box<D> {
    type Value = D::Value;

    fn dim(&self) -> usize { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::Value {
        (**self).sample(rng)
    }
}

impl<'a, D: Space> Space for &'a D {
    type Value = D::Value;

    fn dim(&self) -> usize { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::Value {
        (**self).sample(rng)
    }
}

/// Trait for defining spaces bounded to lie on an interval I.
///
/// Note: If both `inf` and `sup` are well defined (i.e. are not None), then the interval is closed
/// and you have defined a compact space.
pub trait BoundedSpace: Space
where Self::Value: PartialOrd
{
    /// The upper/lower bound type; not necessarily equal to `Space::Value`.
    type BoundValue: PartialOrd + Copy;

    /// Returns the value of the dimension's infimum.
    fn inf(&self) -> Option<Self::BoundValue>;

    /// Returns the value of the dimension's supremum.
    fn sup(&self) -> Option<Self::BoundValue>;

    /// Returns true iff `val` lies within the dimension's bounds (inclusive).
    fn contains(&self, val: Self::BoundValue) -> bool;
}

/// Trait for defining spaces containing a finite set of values.
pub trait FiniteSpace: BoundedSpace
where Self::Value: PartialOrd
{
    /// Returns the finite range of values in this dimension.
    fn range(&self) -> Range<Self::Value>;
}
