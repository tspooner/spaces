#![macro_use]

macro_rules! stationary_kernel {
    ($(#[$attr:meta])* => $name:ident, $self:ident, $dist:ident, $code:block) => {
        use crate::{Vector, norms::l2};
        use super::Kernel;

        $(#[$attr])*
        #[derive(Clone, Copy)]
        pub struct $name {
            pub lengthscale: f64,
        }

        impl $name {
            pub fn new(lengthscale: f64) -> $name {
                $name { lengthscale }
            }

            fn kernel_stationary(&$self, $dist: f64) -> f64 $code
        }

        impl Kernel<f64> for $name {
            fn kernel(&$self, x: &f64, y: &f64) -> f64 {
                $self.kernel_stationary((x - y).abs())
            }
        }

        impl<'a> Kernel<Vector<f64>> for $name {
            fn kernel(&$self, x: &Vector<f64>, y: &Vector<f64>) -> f64 {
                $self.kernel_stationary(l2((x - y).as_slice().unwrap()))
            }
        }
    };
}
