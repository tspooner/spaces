stationary_kernel!(
    /// Exponentiated quadratic kernel.
    => ExpQuad, self, dist, {
        let dist_sq = dist * dist;
        let bandwidth = self.lengthscale * self.lengthscale;

        let sqrt_5 = 5.0f64.sqrt();

        bandwidth * (1.0 + sqrt_5 * dist + 5.0 / 3.0 * dist_sq) * (-sqrt_5 * dist).exp()
    }
);
