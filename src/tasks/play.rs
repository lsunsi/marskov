use {Game, Memory, Policy, Sample};
use std::sync::mpsc::Sender;
use std::sync::RwLock;

pub fn play<G: Game, P: Policy, M: Memory<G::State, G::Action>>(
    game: &mut G,
    policy: &mut P,
    memory: &RwLock<M>,
    sender: &Sender<Sample<G::State, G::Action>>,
) {
    loop {
        let state = game.state();
        let mut action_values = vec![];
        let memory = memory.read().unwrap();

        for action in game.actions() {
            let value = memory.get(&state, &action);
            action_values.push((action, value));
        }

        let action = match policy.choose(action_values) {
            Some(action) => action,
            None => break,
        };

        game.act(&action);
        let reward = game.reward();
        let next_state = game.state();
        let sample = (state, action, next_state, reward);

        if sender.send(sample).is_err() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::counter::*;
    use std::sync::mpsc::channel;
    use std::thread::spawn;
    use policies::Greedy;
    use memories::Table;
    use Memory;

    #[test]
    fn test_play() {
        let (sender, receiver) = channel();
        let mut table: Table<i8, Operation> = Table::default();

        table.set(0, Operation::Inc, 1.);
        table.set(1, Operation::Dec, 1.);

        spawn(move || {
            play(
                &mut Counter::default(),
                &mut Greedy::default(),
                &RwLock::new(table),
                &sender,
            )
        });

        assert_eq!(receiver.recv().unwrap(), (0, Operation::Inc, 1, 1.));
        assert_eq!(receiver.recv().unwrap(), (1, Operation::Dec, 0, -1.));
        assert_eq!(receiver.recv().unwrap(), (0, Operation::Inc, 1, 1.));
        assert_eq!(receiver.recv().unwrap(), (1, Operation::Dec, 0, -1.));
    }
}
