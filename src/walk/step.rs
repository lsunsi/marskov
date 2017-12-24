use {Game, Memory, Policy, Sample};

pub fn step<S, A: Copy, G: Game<S, A>, P: Policy, M: Memory<S, A>>(
    game: &mut G,
    policy: &mut P,
    memory: &M,
) -> Option<Sample<S, A>> {
    let state = game.state();

    let mut actions_values = vec![];
    for action in game.actions() {
        let value = memory.get(&state, &action);
        actions_values.push((action, value));
    }

    if let Some(action) = policy.choose(&actions_values) {
        game.act(action);

        let next_state = game.state();

        return Some((state, *action, next_state, game.reward()));
    }

    None
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

    impl Game<Counter, Operation> for Counter {
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

        let steps = [
            Some((
                Counter { value: 0 },
                Operation::Inc,
                Counter { value: 1 },
                1.,
            )),
            Some((
                Counter { value: 1 },
                Operation::Inc,
                Counter { value: 2 },
                2.,
            )),
            None,
        ];

        for s in steps.iter() {
            assert_eq!(step(&mut game, &mut policy, &memory), *s);
        }
    }
}
