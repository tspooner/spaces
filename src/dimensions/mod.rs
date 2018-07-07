//! 1-dimensional space types module.

mod null;
pub use self::null::Null;

mod infinite;
pub use self::infinite::Infinite;

mod continuous;
pub use self::continuous::Continuous;

mod partitioned;
pub use self::partitioned::Partitioned;

mod natural;
pub use self::natural::Natural;

mod discrete;
pub use self::discrete::Discrete;

mod binary;
pub use self::binary::Binary;
