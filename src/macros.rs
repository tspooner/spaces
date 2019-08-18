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

macro_rules! impl_union_intersect {
    ($type:ty, $build:expr) => {
        impl Union for $type {
            fn union(self, _: &$type) -> Self {
                self
            }
        }

        impl Intersection for $type {
            fn intersect(self, _: &$type) -> Self {
                self
            }
        }
    }
}
