pub trait Policy {
    fn choose<'a, A>(&mut self, &'a [(A, f64)]) -> Option<&'a A>;
}
