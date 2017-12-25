use {Game, Memory, Policy, Sample};

pub fn step<G: Game, P: Policy, M: Memory<G::State, G::Action>>(
    game: &mut G,
    policy: &mut P,
    memory: &M,
) -> Option<Sample<G::State, G::Action>> {
    let state = game.state();

    let mut actions_values = vec![];
    for action in game.actions() {
        let value = memory.get(&state, &action);
        actions_values.push((action, value));
    }

    if let Some(action) = policy.choose(actions_values) {
        game.act(&action);

        let next_state = game.state();

        return Some((state, action, next_state, game.reward()));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::counter::*;
    use memories::Table;
    use policies::Greedy;
    use Memory;

    #[test]
    fn test_step() {
        let mut game = Counter::default();
        let mut policy = Greedy::default();
        let mut memory: Table<i8, Operation> = Table::default();
        memory.set(1, Operation::Dec, 1.0);

        let steps = [
            Some((0, Operation::Inc, 1, 1.)),
            Some((1, Operation::Dec, 0, -1.)),
        ];

        for s in steps.iter() {
            assert_eq!(step(&mut game, &mut policy, &memory), *s);
        }
    }
}
