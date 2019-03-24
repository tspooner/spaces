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
pub type VectorView<'a, T = f64> = ndarray::ArrayView1<'a, T>;
pub type VectorViewMut<'a, T = f64> = ndarray::ArrayViewMut1<'a, T>;

/// 2d array type.
pub type Matrix<T = f64> = ndarray::Array2<T>;
pub type MatrixView<'a, T = f64> = ndarray::ArrayView2<'a, T>;
pub type MatrixViewMut<'a, T = f64> = ndarray::ArrayViewMut2<'a, T>;

mod interval;
pub use self::interval::*;
