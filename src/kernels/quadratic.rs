stationary_kernel!(
    /// Rational quadratic kernel.
    => RationalQuadratic, self, dist, {
        let dist_sq = dist * dist;

        1.0 - dist_sq / (dist_sq + self.lengthscale * self.lengthscale)
    }
);
