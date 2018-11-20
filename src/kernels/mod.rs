// TODO: ANOVA kernel

pub trait Kernel<I: ?Sized>: Clone {
    fn kernel(&self, x: &I, y: &I) -> f64;
}

import_all!(macros);
import_all!(exponential);
import_all!(gaussian);
import_all!(quadratic);
import_all!(exp_quad);
import_all!(cauchy);
import_all!(wave);
