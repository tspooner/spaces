stationary_kernel!(
    /// Cauchy kernel.
    => Cauchy, self, dist, {
        let dist_sq = dist * dist;

        1.0 / (1.0 + dist_sq / self.lengthscale / self.lengthscale)
    }
);
