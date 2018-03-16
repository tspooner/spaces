//! Combination space types module.

mod empty;
pub use self::empty::EmptySpace;

mod pair;
pub use self::pair::PairSpace;

mod regular;
pub use self::regular::RegularSpace;

mod named;
pub use self::named::NamedSpace;
