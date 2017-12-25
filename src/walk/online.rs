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
    use game::counter::*;
    use memories::Table;
    use policies::Greedy;
    use Memory;

    #[test]
    fn test_online() {
        let mut policy = Greedy::default();
        let memory = RwLock::new(Table::default());

        let mut steps = vec![
            (0, Operation::Dec, -1, -1.),
            (1, Operation::Dec, 0, -1.),
            (2, Operation::Dec, 1, -1.),
            (1, Operation::Inc, 2, 1.),
            (0, Operation::Inc, 1, 1.),
        ];

        for s in online(&mut Counter::default(), &mut policy, &memory) {
            match steps.pop() {
                Some(step) => assert_eq!(s, step),
                None => break,
            };

            if s.2 == 2 {
                let mut memory = memory.write().unwrap();
                memory.set(2, Operation::Dec, 1.);
                memory.set(1, Operation::Dec, 1.);
                memory.set(0, Operation::Dec, 1.);
            }
        }
    }
}
