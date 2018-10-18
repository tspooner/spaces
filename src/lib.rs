pub extern crate ndarray;
extern crate rand;

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod macros;

pub mod dimensions;
pub mod norms;

mod card;
pub use self::card::Card;

mod spaces;
pub use self::spaces::*;

/// 1d array type.
pub type Vector<T = f64> = ndarray::Array1<T>;

/// 2d array type.
pub type Matrix<T = f64> = ndarray::Array2<T>;

use std::fmt::Debug;
use std::ops::Range;

/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of the space.
    type Value: Debug + Clone;

    /// Return the number of dimensions in the space.
    fn dim(&self) -> usize;

    /// Return the number of elements in the set composing the space.
    fn card(&self) -> Card;

    /// Generate a random sample from the space.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Value;
}

impl<D: Space> Space for Box<D> {
    type Value = D::Value;

    fn dim(&self) -> usize { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Value {
        (**self).sample(rng)
    }
}

impl<'a, D: Space> Space for &'a D {
    type Value = D::Value;

    fn dim(&self) -> usize { (**self).dim() }

    fn card(&self) -> Card { (**self).card() }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Value {
        (**self).sample(rng)
    }
}

/// Space type bounded on a compact interval I.
pub trait BoundedSpace: Space
where Self::Value: PartialOrd
{
    /// The upper/lower bound type; not necessarily equal to `Space::Value`.
    type BoundValue: PartialOrd + Copy;

    /// Returns the value of the dimension's infimum.
    fn inf(&self) -> Self::BoundValue;

    /// Returns the value of the dimension's supremum.
    fn sup(&self) -> Self::BoundValue;

    /// Returns true iff `val` is within the dimension's bounds.
    fn contains(&self, val: Self::BoundValue) -> bool;
}

/// Space type with bounds and a finite set of values.
pub trait FiniteSpace: BoundedSpace
where Self::Value: PartialOrd
{
    /// Returns the finite range of values in this dimension.
    fn range(&self) -> Range<Self::Value>;
}

/// A trait for types implementing a mapping from values of one set onto
/// another.
pub trait Surjection<X, Y> {
    /// Map value from domain onto codomain.
    fn map(&self, from: X) -> Y;
}
