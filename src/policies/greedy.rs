use Policy;

#[derive(Default)]
pub struct Greedy;

impl Policy for Greedy {
    fn choose<A>(&mut self, mut action_values: Vec<(A, f64)>) -> Option<A> {
        action_values.sort_unstable_by(|&(_, v1), &(_, v2)| v1.partial_cmp(&v2).unwrap());
        action_values.pop().map(|av| av.0)
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
        assert_eq!(greedy.choose(vec![]) as Option<Action>, None);
    }

    #[test]
    fn some_max_valued_action() {
        let mut greedy = Greedy::default();

        let action_values_1 = vec![(Action::Jump, 0.1), (Action::Stay, 0.2)];
        let action_values_2 = vec![(Action::Jump, 0.2), (Action::Stay, 0.1)];

        assert_eq!(greedy.choose(action_values_1), Some(Action::Stay));
        assert_eq!(greedy.choose(action_values_2), Some(Action::Jump));
    }
}
