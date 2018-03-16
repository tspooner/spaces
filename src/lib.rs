extern crate rand;
pub extern crate ndarray;

extern crate serde;
extern crate serde_json;
extern crate serde_test;
#[macro_use] extern crate serde_derive;


mod macros;

pub mod dimensions;
pub mod norms;

mod span;
pub use self::span::Span;

mod spaces;
pub use self::spaces::*;


/// 1d array type.
pub type Vector<T = f64> = ndarray::Array1<T>;

/// 2d array type.
pub type Matrix<T = f64> = ndarray::Array2<T>;


use rand::ThreadRng;
use std::fmt::Debug;
use std::ops::Range;

/// Trait for defining geometric spaces.
pub trait Space {
    /// The data representation of the space.
    type Value: Debug + Clone;

    /// Return the number of dimensions in the space.
    fn dim(&self) -> usize;

    /// Return the number of linear combinations of values in the space.
    fn span(&self) -> Span;

    /// Generate a random sample from the space.
    fn sample(&self, rng: &mut ThreadRng) -> Self::Value;
}

impl<D: Space> Space for Box<D> {
    type Value = D::Value;

    fn dim(&self) -> usize { 1 }

    fn span(&self) -> Span { (**self).span() }

    fn sample(&self, rng: &mut ThreadRng) -> Self::Value { (**self).sample(rng) }
}

impl<'a, D: Space> Space for &'a D {
    type Value = D::Value;

    fn dim(&self) -> usize { 1 }

    fn span(&self) -> Span { (**self).span() }

    fn sample(&self, rng: &mut ThreadRng) -> Self::Value { (**self).sample(rng) }
}

/// Space type with saturating upper/lower bounds.
pub trait BoundedSpace: Space where Self::Value: PartialOrd {
    /// The upper/lower bound type; not necessarily equal to `Space::Value`.
    type BoundValue: PartialOrd + Copy;

    /// Returns a reference to the dimension's lower value bound (inclusive).
    fn lb(&self) -> &Self::BoundValue;

    /// Returns a reference to the dimension's upper value bound (exclusive).
    fn ub(&self) -> &Self::BoundValue;

    /// Returns an owned tuple of the lower and upper bounds on the dimension.
    fn limits(&self) -> (Self::BoundValue, Self::BoundValue) {
        (*self.lb(), *self.ub())
    }

    /// Returns true iff `val` is within the dimension's bounds.
    fn contains(&self, val: Self::BoundValue) -> bool;
}

/// Space type with bounds and a finite set of values.
pub trait FiniteSpace: BoundedSpace where Self::Value: PartialOrd {
    /// Returns the finite range of values in this dimension.
    fn range(&self) -> Range<Self::Value>;
}

/// A trait for types implementing a mapping from values of one set onto another.
pub trait Surjection<X, Y> {
    /// Map value from domain onto codomain.
    fn map(&self, from: X) -> Y;
}
