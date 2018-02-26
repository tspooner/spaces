//! Dimension representations module.

use Span;
use rand::ThreadRng;
use std::fmt::Debug;
use std::ops::Range;


/// The basic dimension type.
pub trait Dimension {
    /// The corresponding primitive type.
    type Value: Debug + Clone;

    /// Sample a random value contained by this dimension.
    fn sample(&self, rng: &mut ThreadRng) -> Self::Value;

    /// Map a compatible input into a valid value of this dimension.
    fn convert(&self, val: f64) -> Self::Value;

    /// Returns the total span of this dimension.
    fn span(&self) -> Span;
}

/// Dimension type with saturating upper/lower bounds.
pub trait BoundedDimension: Dimension where Self::Value: PartialOrd {
    /// The upper/lower bound type; not necessarily equal to `Dimension::Value`.
    type ValueBound: PartialOrd + Copy;

    /// Returns a reference to the dimension's lower value bound (inclusive).
    fn lb(&self) -> &Self::ValueBound;

    /// Returns a reference to the dimension's upper value bound (exclusive).
    fn ub(&self) -> &Self::ValueBound;

    /// Returns an owned tuple of the lower and upper bounds on the dimension.
    fn limits(&self) -> (Self::ValueBound, Self::ValueBound) {
        (*self.lb(), *self.ub())
    }

    /// Returns true iff `val` is within the dimension's bounds.
    fn contains(&self, val: Self::ValueBound) -> bool;

    /// Returns true if either the upper or lower bound are infinite.
    fn is_infinite(&self) -> bool;
}

/// Dimension type with bounds and a finite set of values.
pub trait FiniteDimension: BoundedDimension where Self::Value: PartialOrd {
    /// Returns the finite range of values in this dimension.
    fn range(&self) -> Range<Self::Value>;
}


impl<D: Dimension> Dimension for Box<D> {
    type Value = D::Value;

    fn sample(&self, rng: &mut ThreadRng) -> Self::Value {
        (**self).sample(rng)
    }

    fn convert(&self, val: f64) -> Self::Value {
        (**self).convert(val)
    }

    fn span(&self) -> Span {
        (**self).span()
    }
}

impl<'a, D: Dimension> Dimension for &'a D {
    type Value = D::Value;

    fn sample(&self, rng: &mut ThreadRng) -> Self::Value {
        (**self).sample(rng)
    }

    fn convert(&self, val: f64) -> Self::Value {
        (**self).convert(val)
    }

    fn span(&self) -> Span {
        (**self).span()
    }
}


mod null;
pub use self::null::Null;

mod infinite;
pub use self::infinite::Infinite;

mod continuous;
pub use self::continuous::Continuous;

mod partitioned;
pub use self::partitioned::Partitioned;

mod discrete;
pub use self::discrete::Discrete;
