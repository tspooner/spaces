extern crate rand;
extern crate ndarray;

extern crate serde;
extern crate serde_json;
extern crate serde_test;
#[macro_use] extern crate serde_derive;

extern crate rusty_machine;
pub use self::rusty_machine::learning::toolkit::kernel as kernels;


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


