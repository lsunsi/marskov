pub trait Game<A> {
    fn actions(&self) -> Vec<A>;
    fn reward(&self) -> f64;
    fn act(&mut self, &A);
}
