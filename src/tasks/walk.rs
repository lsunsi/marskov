use {Game, Memory, Policy};

pub struct Walk<'a, G: 'a + Game, M: 'a + Memory<G::State, G::Action>, P: 'a + Policy> {
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a M,
}

impl<'a, G: Game + Clone, M: Memory<G::State, G::Action>, P: Policy> Iterator
    for Walk<'a, G, M, P> {
    type Item = (G::Action, G);

    fn next(&mut self) -> Option<(G::Action, G)> {
        let state = self.game.state();

        let mut action_values = vec![];

        for action in self.game.actions() {
            let value = self.memory.get(&state, &action);
            action_values.push((action, value));
        }

        if let Some(action) = self.policy.choose(action_values) {
            self.game.act(&action);
            return Some((action, self.game.clone()));
        }
        None
    }
}

pub fn walk<'a, G: Game, M: Memory<G::State, G::Action>, P: Policy>(
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a M,
) -> Walk<'a, G, M, P> {
    Walk {
        game: game,
        memory: memory,
        policy: policy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::counter::*;
    use memories::Table;
    use policies::Greedy;

    #[test]
    fn test_walk() {
        let mut game = Counter::default();
        let mut policy = Greedy::default();
        let memory: Table<i8, Operation> = Table::default();

        let mut steps = vec![
            (
                Operation::Inc,
                Counter {
                    current_value: 2,
                    last_value: 1,
                },
            ),
            (
                Operation::Inc,
                Counter {
                    current_value: 1,
                    last_value: 0,
                },
            ),
        ];

        for s in walk(&mut game, &mut policy, &memory) {
            match steps.pop() {
                Some(step) => assert_eq!(s, step),
                None => break,
            }
        }
    }
}
