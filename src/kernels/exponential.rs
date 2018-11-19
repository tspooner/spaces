stationary_kernel!(
    /// Exponential kernel.
    => Exponential, self, dist, {
        (-dist.abs() / self.lengthscale / self.lengthscale).exp()
    }
);
