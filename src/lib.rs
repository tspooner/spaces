//! Set/space primitives for defining machine learning problems.
//!
//! `spaces` provides set/space primitives to be used for defining properties of
//! machine learning problems. Traits such as `Space`, and it's derivatives, may
//! be used to define state/action spaces, for example. Mappings between
//! different spaces may also be defined using traits such as `Surjection` to
//! streamline many common preprocessing and type conversion tasks.
extern crate ndarray;
extern crate num_traits;

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod macros;

import_all!(core);

pub mod continuous;
pub mod discrete;
pub mod misc;
pub mod norms;
pub mod product;
pub mod kernels;
