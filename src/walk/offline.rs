use {walk, Game, Memory, Policy, Sample};

pub struct Walk<'a, G: 'a + Game, M: 'a + Memory<G::State, G::Action>, P: 'a + Policy> {
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a M,
}

impl<'a, G: Game, M: Memory<G::State, G::Action>, P: Policy> Iterator for Walk<'a, G, M, P> {
    type Item = Sample<G::State, G::Action>;

    fn next(&mut self) -> Option<Sample<G::State, G::Action>> {
        walk::step(self.game, self.policy, self.memory)
    }
}

pub fn offline<'a, G: Game, M: Memory<G::State, G::Action>, P: Policy>(
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a M,
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
            if self.value < 2 {
                vec![Operation::Inc]
            } else {
                vec![]
            }
        }

        fn act(&mut self, op: &Operation) {
            if *op == Operation::Inc {
                self.value += 1;
            }
        }

        fn reward(&self) -> f64 {
            self.value as f64
        }
    }

    #[test]
    fn test() {
        let mut game = Counter::default();
        let mut policy = Greedy::default();
        let memory: Table<Counter, Operation> = Table::default();

        let mut steps = vec![
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

        for s in offline(&mut game, &mut policy, &memory) {
            assert_eq!(s, steps.pop().unwrap());
        }

        assert_eq!(steps.len(), 0);
    }
}
