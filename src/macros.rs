#![macro_use]
#![allow(unused_macros)]

macro_rules! clip {
    ($lb:expr, $x:expr, $ub:expr) => {{
        $lb.max($ub.min($x))
    }};
}
