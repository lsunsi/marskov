use std::marker::PhantomData;
use std::sync::RwLock;
use walk::step::step;
use std::ops::Deref;
use memory::Memory;
use policy::Policy;
use sample::Sample;
use game::Game;

pub struct Walk<
    'a,
    S: 'a,
    A: 'a + Copy,
    G: 'a + Game<A> + Into<S> + Clone,
    M: 'a + Memory<S, A>,
    P: 'a + Policy,
> {
    _a: PhantomData<A>,
    _s: PhantomData<S>,
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a RwLock<M>,
}

impl<'a, S, A: Copy, G: Game<A> + Into<S> + Clone, M: Memory<S, A>, P: Policy> Iterator
    for Walk<'a, S, A, G, M, P> {
    type Item = Sample<S, A>;

    fn next(&mut self) -> Option<Sample<S, A>> {
        match self.memory.read() {
            Ok(memory) => step(self.game, self.policy, memory.deref()),
            Err(_) => None,
        }
    }
}

pub fn online<
    'a,
    S: 'a,
    A: 'a + Copy,
    G: 'a + Game<A> + Into<S> + Clone,
    M: Memory<S, A>,
    P: 'a + Policy,
>(
    game: &'a mut G,
    policy: &'a mut P,
    memory: &'a RwLock<M>,
) -> Walk<'a, S, A, G, M, P> {
    Walk {
        _a: PhantomData::default(),
        _s: PhantomData::default(),
        game: game,
        policy: policy,
        memory: memory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memories::table::Table;
    use policies::greedy::Greedy;

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

    impl Game<Operation> for Counter {
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
