use memory::Memory;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Default)]
pub struct Table<S: Eq + Hash, A> {
  map: HashMap<S, HashMap<A, f64>>,
  seed: f64,
}

impl<S: Eq + Hash, A> Table<S, A> {
  pub fn with_seed(seed: f64) -> Table<S, A> {
    let map = HashMap::default();
    Table {
      seed: seed,
      map: map,
    }
  }
}

impl<S: Eq + Hash, A: Eq + Hash> Memory<S, A> for Table<S, A> {
  fn get(&self, state: &S, action: &A) -> f64 {
    *self
      .map
      .get(state)
      .and_then(|map| map.get(action))
      .unwrap_or(&self.seed)
  }

  fn set(&mut self, state: S, action: A, value: f64) {
    if self.map.contains_key(&state) {
      let map = self.map.get_mut(&state).unwrap();
      map.insert(action, value);
    } else {
      let mut map = HashMap::default();
      map.insert(action, value);
      self.map.insert(state, map);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Table;
  use memory::Memory;

  #[test]
  fn seed_get_set_get() {
    let mut table = Table::with_seed(1.3);

    assert_eq!(table.get(&2, &4), 1.3);
    assert_eq!(table.get(&4, &2), 1.3);

    table.set(4, 2, 3.1);

    assert_eq!(table.get(&2, &4), 1.3);
    assert_eq!(table.get(&4, &2), 3.1);
  }

  #[test]
  fn default_seeds_zero() {
    let table = Table::default();
    assert_eq!(table.get(&4, &2), 0.0);
  }
}
