pub trait Game {
    type State;
    type Action;

    fn actions(&self) -> Vec<Self::Action>;
    fn reward(&self) -> f64;
    fn act(&mut self, &Self::Action);
    fn state(&self) -> Self::State;
}
