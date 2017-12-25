pub trait Game {
    type State;
    type Action;

    fn actions(&self) -> Vec<Self::Action>;
    fn reward(&self) -> f64;
    fn act(&mut self, &Self::Action);
    fn state(&self) -> Self::State;
}

#[cfg(test)]
pub mod counter {
    use super::Game;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Operation {
        Inc,
        Dec,
    }

    impl Default for Operation {
        fn default() -> Operation {
            Operation::Dec
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Counter {
        pub last_value: i8,
        pub current_value: i8,
    }

    impl Game for Counter {
        type Action = Operation;
        type State = i8;

        fn state(&self) -> i8 {
            self.current_value
        }

        fn reward(&self) -> f64 {
            (self.current_value - self.last_value) as f64
        }

        fn actions(&self) -> Vec<Operation> {
            vec![Operation::Dec, Operation::Inc]
        }

        fn act(&mut self, operation: &Operation) {
            self.last_value = self.current_value;
            self.current_value += match *operation {
                Operation::Dec => -1,
                Operation::Inc => 1,
            }
        }
    }
}
