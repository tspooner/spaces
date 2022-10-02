//! Module for real scalar spaces.
use crate::intervals;
use num_traits::real::Real;

/// Build a space representing the set of all real numbers.
pub fn reals<V: Real>() -> Reals<V> { intervals::Interval::unbounded() }

pub type Reals<V> = intervals::Unbounded<V>;

/// Build a space representing the set of non-negative real numbers.
pub fn non_negative_reals<V: Real>() -> NonNegativeReals<V> {
    intervals::Interval::left_closed(V::zero())
}

pub type NonNegativeReals<V> = intervals::LeftClosed<V>;

/// Build a space representing the set of strictly positive real numbers.
pub fn positive_reals<V: Real>() -> PositiveReals<V> { intervals::Interval::left_open(V::zero()) }

pub type PositiveReals<V> = intervals::LeftOpen<V>;

/// Build a space representing the set of non-positive real numbers.
pub fn non_positive_reals<V: Real>() -> NonPositiveReals<V> {
    intervals::Interval::right_closed(V::zero())
}

pub type NonPositiveReals<V> = intervals::RightClosed<V>;

/// Build a space representing the set of strictly negative real numbers.
pub fn negative_reals<V: Real>() -> NegativeReals<V> { intervals::Interval::right_open(V::zero()) }

pub type NegativeReals<V> = intervals::RightOpen<V>;
