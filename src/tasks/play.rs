use std::sync::mpsc::Sender;
use std::sync::RwLock;
use std::sync::Arc;
use policy::Policy;
use memory::Memory;
use sample::Sample;
use game::Game;

pub fn play<S: Copy, A: Copy, G: Game<A> + Into<S> + Clone, P: Policy, M: Memory<S, A>>(
  mut game: G,
  mut policy: P,
  memory: Arc<RwLock<M>>,
  sender: Sender<Sample<S, A>>,
) {
  let mut state = game.clone().into();

  loop {
    let result = memory.read();

    if result.is_err() {
      break;
    }

    let memory = result.unwrap();
    let mut action_values = vec![];

    for action in game.actions() {
      let value = memory.get(&state, &action);
      action_values.push((action, value));
    }

    let action = policy.choose(&action_values).unwrap();
    game.act(action);

    let next_state = game.clone().into();
    let sample = (state, *action, next_state, game.reward());
    state = next_state;

    if let Err(_) = sender.send(sample) {
      break;
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::thread::spawn;
  use memories::table::Table;
  use std::sync::mpsc::channel;

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
    let (sender, receiver) = channel();
    let table: Table<i8, Operation> = Table::default();
    let memory = Arc::new(RwLock::new(table));

    let memory_clone = memory.clone();
    spawn(|| play(Counter::default(), First::default(), memory_clone, sender));

    assert_eq!(receiver.recv().unwrap(), (0, Operation::Inc, 1, 0.1));
    assert_eq!(receiver.recv().unwrap(), (1, Operation::Inc, 2, 0.2));
    assert_eq!(receiver.recv().unwrap(), (2, Operation::Dec, 1, 0.1));
    assert_eq!(receiver.recv().unwrap(), (1, Operation::Dec, 0, 0.0));
  }
}