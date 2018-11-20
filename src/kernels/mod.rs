// TODO: ANOVA kernel

pub trait Kernel<I: ?Sized>: Clone {
    fn kernel(&self, x: &I, y: &I) -> f64;
}

import_all!(stationary);
import_all!(rational_quadratic);
