extern crate rand;
pub extern crate ndarray;

extern crate serde;
extern crate serde_json;
extern crate serde_test;
#[macro_use] extern crate serde_derive;


mod macros;

pub mod dimensions;
pub use self::dimensions::{Dimension, BoundedDimension, FiniteDimension};

pub mod norms;

mod span;
pub use self::span::Span;

mod spaces;
pub use self::spaces::*;


/// 1d array type.
pub type Vector<T = f64> = ndarray::Array1<T>;

/// 2d array type.
pub type Matrix<T = f64> = ndarray::Array2<T>;


/// A trait for types implementing a mapping from values of one set onto another.
pub trait Surjection<X, Y> {
    /// Map value from domain onto codomain.
    fn map(&self, from: X) -> Y;
}
