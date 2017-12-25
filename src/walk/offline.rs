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
    use game::counter::*;
    use memories::Table;
    use policies::Greedy;

    #[test]
    fn test_offline() {
        let mut game = Counter::default();
        let mut policy = Greedy::default();
        let memory: Table<i8, Operation> = Table::default();

        let mut steps = vec![(1, Operation::Inc, 2, 1.), (0, Operation::Inc, 1, 1.)];

        for s in offline(&mut game, &mut policy, &memory) {
            match steps.pop() {
                Some(step) => assert_eq!(s, step),
                None => break,
            }
        }
    }
}
