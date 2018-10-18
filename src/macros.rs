#![macro_use]
#![allow(unused_macros)]

macro_rules! clip {
    ($lb:expr, $x:expr, $ub:expr) => {{
        $lb.max($ub.min($x))
    }};
}

macro_rules! import_all {
    ($module:ident) => {
        mod $module;
        pub use self::$module::*;
    };
}
