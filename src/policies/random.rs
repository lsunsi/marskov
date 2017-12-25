use Policy;
use rand::{thread_rng, Rng};

pub struct Random {
    rng: Box<Rng>,
}

impl Default for Random {
    fn default() -> Random {
        Random {
            rng: Box::new(thread_rng()),
        }
    }
}

impl Policy for Random {
    fn choose<A>(&mut self, mut action_values: Vec<(A, f64)>) -> Option<A> {
        match action_values.len() {
            0 => None,
            n => Some(action_values.swap_remove(self.rng.gen_range(0, n)).0),
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, StdRng};
    use game::counter::*;
    use super::*;

    #[test]
    fn none_for_empty_action_values() {
        let mut random = Random::default();
        assert_eq!(random.choose(vec![]) as Option<Operation>, None);
    }

    #[test]
    fn some_random_action() {
        let mut random1 = Random {
            rng: Box::new(StdRng::from_seed(&[0])),
        };
        let mut random2 = Random {
            rng: Box::new(StdRng::from_seed(&[4])),
        };

        let action_values = vec![(Operation::Dec, 0.1), (Operation::Inc, 0.2)];

        assert_eq!(random1.choose(action_values.clone()), Some(Operation::Inc));
        assert_eq!(random2.choose(action_values.clone()), Some(Operation::Dec));
    }
}
