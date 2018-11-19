stationary_kernel!(
    /// Isotropic Guassian kernel.
    => IsotropicGaussian, self, dist, {
        (-dist * dist / self.lengthscale / self.lengthscale).exp()
    }
);
