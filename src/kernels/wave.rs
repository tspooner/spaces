stationary_kernel!(
    /// Wave kernel.
    ///
    /// Note: positive definite in R^3.
    => Wave, self, dist, {
        let dist = dist.abs();
        let bandwidth = self.lengthscale * self.lengthscale;

        bandwidth / dist * (dist / bandwidth).sin()
    }
);
