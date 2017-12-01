use policy::Policy;

#[derive(Default)]
struct Greedy;

impl Policy for Greedy {
    fn choose<'a, A>(&mut self, action_values: &'a [(A, f64)]) -> Option<&'a A> {
        action_values
            .iter()
            .max_by(|av1, av2| av1.1.partial_cmp(&av2.1).unwrap())
            .map(|av| &av.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Action {
        Jump,
        Stay,
    }

    #[test]
    fn none_for_empty_action_values() {
        let mut greedy = Greedy::default();
        assert_eq!(greedy.choose(&[]) as Option<&Action>, None);
    }

    #[test]
    fn some_max_valued_action() {
        let mut greedy = Greedy::default();

        let action_values_1 = [(Action::Jump, 0.1), (Action::Stay, 0.2)];
        let action_values_2 = [(Action::Jump, 0.2), (Action::Stay, 0.1)];

        assert_eq!(greedy.choose(&action_values_1), Some(&Action::Stay));
        assert_eq!(greedy.choose(&action_values_2), Some(&Action::Jump));
    }
}
