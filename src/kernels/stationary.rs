macro_rules! stationary_kernel {
    ($(#[$attr:meta])* => $name:ident, $self:ident, $r:ident, $code:block) => {
        $(#[$attr])*
        #[derive(Clone)]
        pub struct $name {
            pub variance: f64,
            pub lengthscales: $crate::Vector<f64>,
        }

        impl $name {
            pub fn new(variance: f64, lengthscales: $crate::Vector<f64>) -> $name {
                $name { variance, lengthscales }
            }

            pub fn non_ard(variance: f64, lengthscale: f64) -> $name {
                $name::new(variance, $crate::Vector::from_vec(vec![lengthscale]))
            }

            fn kernel_stationary(&$self, $r: f64) -> f64 $code
        }

        impl Default for $name {
            fn default() -> $name {
                $name::non_ard(1.0, 1.0)
            }
        }

        impl $crate::kernels::Kernel<f64> for $name {
            fn kernel(&$self, x: &f64, y: &f64) -> f64 {
                $self.kernel_stationary((x - y).abs() / $self.lengthscales[0])
            }
        }

        impl $crate::kernels::Kernel<$crate::Vector<f64>> for $name {
            fn kernel(&$self, x: &$crate::Vector<f64>, y: &$crate::Vector<f64>) -> f64 {
                let scaled_diff = (x - y) / &$self.lengthscales;

                $self.kernel_stationary($crate::norms::l2(scaled_diff.as_slice().unwrap()))
            }
        }
    };
}

stationary_kernel!(
    /// Exponential kernel.
    => Exponential, self, r, {
        self.variance * (-r).exp()
    }
);

stationary_kernel!(
    /// Matern 3/2 kernel.
    => Matern32, self, r, {
        let sqrt_3 = 3.0f64.sqrt();

        self.variance * (1.0 + sqrt_3 * r) * (-sqrt_3 * r).exp()
    }
);

stationary_kernel!(
    /// Matern 5/2 kernel.
    => Matern52, self, r, {
        let sqrt_5 = 3.0f64.sqrt();

        self.variance * (1.0 + sqrt_5 * r + 5.0 / 3.0 * r * r) * (-sqrt_5 * r).exp()
    }
);

stationary_kernel!(
    /// Exponentiated quadratic kernel.
    => ExpQuad, self, r, {
        self.variance * (-0.5 * r * r).exp()
    }
);

stationary_kernel!(
    /// Cosine kernel.
    => Cosine, self, r, {
        self.variance * r.cos()
    }
);

pub type RBF = ExpQuad;
pub type Guassian = ExpQuad;
