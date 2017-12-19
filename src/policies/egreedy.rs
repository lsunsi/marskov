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
    fn choose<'a, A>(&mut self, action_values: &'a [(A, f64)]) -> Option<&'a A> {
        if self.rng.gen::<f64>() < self.epsilon {
            self.rng.choose(action_values).map(|av| &av.0)
        } else {
            action_values
                .iter()
                .max_by(|av1, av2| av1.1.partial_cmp(&av2.1).unwrap())
                .map(|av| &av.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, StdRng};
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Action {
        Jump,
        Stay,
    }

    #[test]
    fn none_for_empty_action_values() {
        let mut egreedy = Egreedy::new(0.5);
        assert_eq!(egreedy.choose(&[]) as Option<&Action>, None);
    }

    #[test]
    fn some_random_action_if_epsilon_max() {
        let mut egreedy = Egreedy {
            epsilon: 1.0,
            rng: Box::new(StdRng::from_seed(&[1])),
        };

        let action_values = [(Action::Jump, 0.1), (Action::Stay, 0.2)];

        assert_eq!(egreedy.choose(&action_values), Some(&Action::Jump));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Jump));
    }

    #[test]
    fn some_greedy_action_if_epsilon_min() {
        let mut egreedy = Egreedy {
            epsilon: 0.0,
            rng: Box::new(StdRng::from_seed(&[1])),
        };

        let action_values = [(Action::Jump, 0.1), (Action::Stay, 0.2)];

        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
    }

    #[test]
    fn some_egreedy_action_if_epsilon_half() {
        let mut egreedy = Egreedy {
            epsilon: 0.5,
            rng: Box::new(StdRng::from_seed(&[1])),
        };

        let action_values = [(Action::Jump, 0.1), (Action::Stay, 0.2)];

        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Jump));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
        assert_eq!(egreedy.choose(&action_values), Some(&Action::Stay));
    }
}
