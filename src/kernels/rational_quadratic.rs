#[derive(Clone)]
pub struct RationalQuadratic {
    pub variance: f64,
    pub lengthscales: crate::Vector<f64>,

    pub power: f64,
}

impl RationalQuadratic {
    pub fn new(variance: f64, lengthscales: crate::Vector<f64>, power: f64) -> RationalQuadratic {
        RationalQuadratic { power, variance, lengthscales }
    }

    pub fn non_ard(power: f64, variance: f64, lengthscale: f64) -> RationalQuadratic {
        RationalQuadratic::new(variance, crate::Vector::from_vec(vec![lengthscale]), power)
    }

    fn kernel_stationary(&self, r: f64) -> f64 {
        self.variance * (-self.power * (r * r / 2.0).ln_1p()).exp()
    }
}

impl Default for RationalQuadratic {
    fn default() -> RationalQuadratic {
        RationalQuadratic::non_ard(1.0, 1.0, 2.0)
    }
}

impl crate::kernels::Kernel<f64> for RationalQuadratic {
    fn kernel(&self, x: &f64, y: &f64) -> f64 {
        self.kernel_stationary((x - y).abs() / self.lengthscales[0])
    }
}

impl crate::kernels::Kernel<[f64]> for RationalQuadratic {
    fn kernel(&self, x: &[f64], y: &[f64]) -> f64 {
        self.kernel_stationary(x
            .into_iter()
            .zip(y.into_iter())
            .map(|(x, y)| x - y)
            .zip(self.lengthscales.iter())
            .map(|(d, l)| d / l)
            .fold(0.0, |acc, z| acc + z * z)
            .sqrt())
    }
}


impl crate::kernels::Kernel<crate::Vector<f64>> for RationalQuadratic {
    fn kernel(&self, x: &crate::Vector<f64>, y: &crate::Vector<f64>) -> f64 {
        let scaled_diff = (x - y) / &self.lengthscales;

        self.kernel_stationary(crate::norms::l2(scaled_diff.as_slice().unwrap()))
    }
}

