pub trait Memory<S, A> {
    fn get(&self, &S, &A) -> f64;
    fn set(&mut self, S, A, f64);
}
