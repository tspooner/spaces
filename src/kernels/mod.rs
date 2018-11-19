pub trait Kernel<I: ?Sized>: Clone {
    fn kernel(&self, x: &I, y: &I) -> f64;
}
