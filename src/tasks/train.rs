use {Brain, Game, Memory, Policy, Sample};
use std::sync::mpsc::Receiver;
use std::sync::RwLock;

pub fn train<G: Game, P: Policy, M: Memory<G::State, G::Action>>(
    game: &G,
    policy: &mut P,
    memory: &RwLock<M>,
    receiver: &Receiver<Sample<G::State, G::Action>>,
    brain: &Brain,
) {
    while let Ok((state0, action0, state1, reward)) = receiver.recv() {
        let mut memory = match memory.write() {
            Ok(memory) => memory,
            Err(_) => break,
        };

        let mut action_values = vec![];

        for action in game.actions() {
            let value = memory.get(&state1, &action);
            action_values.push((action, value));
        }

        let action1 = policy.choose(action_values).unwrap();

        let value0 = memory.get(&state0, &action0);
        let value1 = memory.get(&state1, &action1);

        memory.set(state0, action0, brain.learn(value0, value1, reward));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::counter::*;
    use std::sync::mpsc::sync_channel;
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    use policies::Greedy;
    use memories::Table;
    use std::sync::Arc;

    #[test]
    fn test_train() {
        let (sender, receiver) = sync_channel(0);
        let table: Table<i8, Operation> = Table::default();
        let memory = Arc::new(RwLock::new(table));

        let training_memory = memory.clone();
        spawn(move || {
            train(
                &Counter::default(),
                &mut Greedy::default(),
                &training_memory,
                &receiver,
                &Brain::new(0.5, 0.5),
            )
        });

        assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 0.0);
        assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 0.0);
        sender.send((0, Operation::Inc, 1, 4.0)).unwrap();
        sleep(Duration::from_millis(1));
        assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 2.0);
        assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 0.0);
        sender.send((1, Operation::Dec, 0, 4.0)).unwrap();
        sleep(Duration::from_millis(1));
        assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 2.0);
        assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 2.5);
        sender.send((0, Operation::Inc, 1, 4.0)).unwrap();
        sleep(Duration::from_millis(1));
        assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 3.625);
        assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 2.5);
    }
}
