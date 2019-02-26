#![macro_use]

mod card;
pub use self::card::Card;

mod space;
pub use self::space::*;

mod mapping;
pub use self::mapping::*;

mod enclosing;
pub use self::enclosing::*;

/// 1d array type.
pub type Vector<T = f64> = ndarray::Array1<T>;

/// 2d array type.
pub type Matrix<T = f64> = ndarray::Array2<T>;

mod interval;
pub use self::interval::*;
