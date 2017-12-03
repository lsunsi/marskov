use std::sync::mpsc::Receiver;
use std::sync::RwLock;
use std::sync::Arc;
use policy::Policy;
use memory::Memory;
use sample::Sample;
use brain::Brain;
use game::Game;

pub fn train<S: Copy, A: Copy, G: Game<A> + Into<S> + Clone, P: Policy, M: Memory<S, A>>(
  game: G,
  mut policy: P,
  memory: Arc<RwLock<M>>,
  receiver: Receiver<Sample<S, A>>,
  brain: Brain,
) {
  while let Ok((state0, action0, state1, reward)) = receiver.recv() {
    let result = memory.write();

    if let Err(_) = result {
      break;
    }

    let mut memory = result.unwrap();
    let mut action_values = vec![];

    for action in game.actions() {
      let value = memory.get(&state1, &action);
      action_values.push((action, value));
    }

    let action1 = policy.choose(&action_values).unwrap();

    let value0 = memory.get(&state0, &action0);
    let value1 = memory.get(&state1, &action1);

    memory.set(state0, action0, brain.learn(value0, value1, reward));
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::thread::{spawn, sleep_ms};
  use std::sync::mpsc::sync_channel;
  use memories::table::Table;

  #[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
  enum Operation {
    Inc,
    Dec,
  }

  impl Default for Operation {
    fn default() -> Operation {
      Operation::Inc
    }
  }

  #[derive(Clone)]
  struct Counter {
    value: i8,
    up: bool,
  }

  impl Default for Counter {
    fn default() -> Counter {
      Counter { value: 0, up: true }
    }
  }

  impl Into<i8> for Counter {
    fn into(self) -> i8 {
      self.value
    }
  }

  impl Game<Operation> for Counter {
    fn actions(&self) -> Vec<Operation> {
      if self.up {
        vec![Operation::Inc]
      } else {
        vec![Operation::Dec]
      }
    }

    fn act(&mut self, op: &Operation) {
      self.value += match *op {
        Operation::Dec => -1,
        Operation::Inc => 1,
      };

      self.up = match self.value {
        2 => false,
        0 => true,
        _ => self.up,
      };
    }

    fn reward(&self) -> f64 {
      if self.value != 0 {
        (self.value as f64) / 10.0
      } else {
        0.0
      }
    }
  }

  #[derive(Default)]
  struct First;
  impl Policy for First {
    fn choose<'a, A>(&mut self, action_values: &'a [(A, f64)]) -> Option<&'a A> {
      action_values.first().map(|av| &av.0)
    }
  }

  #[test]
  fn test() {
    let (sender, receiver) = sync_channel(0);
    let table: Table<i8, Operation> = Table::default();
    let memory = Arc::new(RwLock::new(table));

    let memory_clone = memory.clone();
    spawn(|| {
      train(
        Counter::default(),
        First::default(),
        memory_clone,
        receiver,
        Brain::new(0.5, 0.5),
      )
    });

    assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 0.0);
    assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 0.0);
    sender.send((0, Operation::Inc, 1, 4.0)).unwrap();
    sleep_ms(1);
    assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 2.0);
    assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 0.0);
    sender.send((1, Operation::Dec, 0, 4.0)).unwrap();
    sleep_ms(1);
    assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 2.0);
    assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 2.5);
    sender.send((0, Operation::Inc, 1, 4.0)).unwrap();
    sleep_ms(1);
    assert_eq!(memory.read().unwrap().get(&0, &Operation::Inc), 3.0);
    assert_eq!(memory.read().unwrap().get(&1, &Operation::Dec), 2.5);
  }
}
