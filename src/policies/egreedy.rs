use Policy;
use rand::{thread_rng, Rng};

pub struct Egreedy {
    epsilon: f64,
    rng: Box<Rng>,
}

impl Egreedy {
    pub fn new(epsilon: f64) -> Egreedy {
        Egreedy {
            epsilon: epsilon,
            rng: Box::new(thread_rng()),
        }
    }
}

impl Policy for Egreedy {
    fn choose<A>(&mut self, mut action_values: Vec<(A, f64)>) -> Option<A> {
        if self.rng.gen::<f64>() < self.epsilon {
            match action_values.len() {
                0 => None,
                n => Some(action_values.swap_remove(self.rng.gen_range(0, n)).0),
            }
        } else {
            action_values.sort_unstable_by(|&(_, v1), &(_, v2)| v1.partial_cmp(&v2).unwrap());
            action_values.pop().map(|(a, _)| a)
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, StdRng};
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    enum Action {
        Jump,
        Stay,
    }

    #[test]
    fn none_for_empty_action_values() {
        let mut egreedy = Egreedy::new(0.5);
        assert_eq!(egreedy.choose(vec![]) as Option<Action>, None);
    }

    #[test]
    fn some_random_action_if_epsilon_max() {
        let mut egreedy = Egreedy {
            epsilon: 1.0,
            rng: Box::new(StdRng::from_seed(&[1])),
        };

        let action_values = vec![(Action::Jump, 0.1), (Action::Stay, 0.2)];

        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Jump));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Jump));
    }

    #[test]
    fn some_greedy_action_if_epsilon_min() {
        let mut egreedy = Egreedy {
            epsilon: 0.0,
            rng: Box::new(StdRng::from_seed(&[1])),
        };

        let action_values = vec![(Action::Jump, 0.1), (Action::Stay, 0.2)];

        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
    }

    #[test]
    fn some_egreedy_action_if_epsilon_half() {
        let mut egreedy = Egreedy {
            epsilon: 0.5,
            rng: Box::new(StdRng::from_seed(&[1])),
        };

        let action_values = vec![(Action::Jump, 0.1), (Action::Stay, 0.2)];

        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Jump));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
        assert_eq!(egreedy.choose(action_values.clone()), Some(Action::Stay));
    }
}
