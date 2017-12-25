pub trait Policy {
    fn choose<A>(&mut self, Vec<(A, f64)>) -> Option<A>;
}
