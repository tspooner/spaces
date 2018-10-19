extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod macros;

import_all!(core);

pub mod continuous;
pub mod discrete;
pub mod norms;
pub mod product;
