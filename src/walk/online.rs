use {walk, Game, Memory, Policy, Sample};
use std::sync::RwLock;
use std::ops::Deref;

pub struct Walk<'a, G: 'a + Game, M: 'a + Memory<G::State, G::Action>, P: 'a + Policy> {
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a RwLock<M>,
}

impl<'a, G: Game, M: Memory<G::State, G::Action>, P: Policy> Iterator for Walk<'a, G, M, P> {
    type Item = Sample<G::State, G::Action>;

    fn next(&mut self) -> Option<Sample<G::State, G::Action>> {
        match self.memory.read() {
            Ok(memory) => walk::step(self.game, self.policy, memory.deref()),
            Err(_) => None,
        }
    }
}

pub fn online<'a, G: 'a + Game, M: Memory<G::State, G::Action>, P: 'a + Policy>(
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a RwLock<M>,
) -> Walk<'a, G, M, P> {
    Walk {
        game: game,
        policy: policy,
        memory: memory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memories::Table;
    use policies::Greedy;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum Operation {
        Inc,
        Dec,
    }

    impl Default for Operation {
        fn default() -> Operation {
            Operation::Inc
        }
    }

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    struct Counter {
        value: i8,
    }

    impl Game for Counter {
        type State = Counter;
        type Action = Operation;

        fn state(&self) -> Counter {
            *self
        }

        fn actions(&self) -> Vec<Operation> {
            if self.value < 2 && self.value > -2 {
                vec![Operation::Dec, Operation::Inc]
            } else {
                vec![]
            }
        }

        fn act(&mut self, op: &Operation) {
            self.value += match *op {
                Operation::Inc => 1,
                Operation::Dec => -1,
            }
        }

        fn reward(&self) -> f64 {
            self.value as f64
        }
    }

    #[test]
    fn test() {
        let mut policy = Greedy::default();
        let memory = RwLock::new(Table::default());

        let mut steps = vec![
            (
                Counter { value: -1 },
                Operation::Dec,
                Counter { value: -2 },
                -2.,
            ),
            (
                Counter { value: 0 },
                Operation::Dec,
                Counter { value: -1 },
                -1.,
            ),
            (
                Counter { value: 1 },
                Operation::Inc,
                Counter { value: 2 },
                2.,
            ),
            (
                Counter { value: 0 },
                Operation::Inc,
                Counter { value: 1 },
                1.,
            ),
        ];

        for s in online(&mut Counter::default(), &mut policy, &memory) {
            assert_eq!(s, steps.pop().unwrap());
        }

        memory
            .write()
            .unwrap()
            .set(Counter { value: 0 }, Operation::Dec, 1.);
        memory
            .write()
            .unwrap()
            .set(Counter { value: -1 }, Operation::Dec, 1.);

        for s in online(&mut Counter::default(), &mut policy, &memory) {
            assert_eq!(s, steps.pop().unwrap());
        }

        assert_eq!(steps.len(), 0);
    }
}
