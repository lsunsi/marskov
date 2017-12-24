extern crate marskov;
extern crate rand;

use std::sync::Arc;
use std::ops::Deref;
use std::sync::RwLock;
use std::time::Duration;
use rand::{thread_rng, Rng};
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};

use marskov::memories::Table;
use marskov::{walk, Brain, Game};
use marskov::tasks::{play, train};
use marskov::policies::{Egreedy, Greedy};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Janete = -1,
    Empty = 0,
    Robson = 1,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Empty
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Board {
    tiles: [Tile; 9],
    invalid: bool,
    winner: Tile,
    count: i8,
}

impl Game<Board, i8> for Board {
    fn state(&self) -> Board {
        *self
    }

    fn actions(&self) -> Vec<i8> {
        if self.invalid || self.winner != Tile::Empty || self.count == 9 {
            vec![-1]
        } else {
            (0..9).collect()
        }
    }

    fn act(&mut self, m: &i8) {
        if *m == -1 {
            self.tiles = [Tile::Empty; 9];
            self.invalid = false;
            self.count = 0;
            self.winner = Tile::Empty;
            return;
        }

        if self.tiles[*m as usize] != Tile::Empty || self.count == 9 {
            self.invalid = true;
            return;
        }

        self.count += 1;
        self.tiles[*m as usize] = Tile::Robson;

        if self.count < 9 {
            self.count += 1;
            loop {
                let m = thread_rng().gen_range(0, 9);
                if self.tiles[m] == Tile::Empty {
                    self.tiles[m] = Tile::Janete;
                    break;
                }
            }
        }

        if self.count == 9 {
            let mut dia1 = 0;
            let mut dia2 = 0;
            for i in 0..3 {
                dia1 += self.tiles[i + 3 * i] as i8;
                dia2 += self.tiles[2 + 2 * i] as i8;
            }
            if dia1 == 3 || dia2 == 3 {
                self.winner = Tile::Robson;
                return;
            }
            if dia1 == -3 || dia2 == -3 {
                self.winner = Tile::Janete;
                return;
            }

            for i in 0..3 {
                let mut row = 0;
                let mut col = 0;
                for j in 0..3 {
                    row += self.tiles[i + 3 * j] as i8;
                    col += self.tiles[j + 3 * i] as i8;
                }
                if row == 3 || col == 3 {
                    self.winner = Tile::Robson;
                    return;
                }
                if row == -3 || col == -3 {
                    self.winner = Tile::Janete;
                    return;
                }
            }
        }
    }

    fn reward(&self) -> f64 {
        if self.invalid {
            return -1.;
        }

        if self.count < 9 {
            return 0.;
        }

        match self.winner {
            Tile::Robson => 1.,
            Tile::Janete => -1.,
            Tile::Empty => -0.1,
        }
    }
}

#[test]
fn solves_tictactoe() {
    let (sender, receiver) = channel();
    let table: Table<Board, i8> = Table::default();
    let memory = Arc::new(RwLock::new(table));

    let training_memory = memory.clone();
    spawn(move || {
        train(
            &Board::default(),
            &mut Greedy::default(),
            &training_memory,
            &receiver,
            &Brain::new(0.99, 0.99),
        )
    });

    let playing_memory = memory.clone();
    spawn(move || {
        play(
            &mut Board::default(),
            &mut Egreedy::new(0.01),
            playing_memory.deref(),
            &sender,
        )
    });

    sleep(Duration::from_secs(60));

    let memory = memory.read().unwrap();
    let mut greedy = Greedy::default();
    let mut board = Board::default();

    let mut total: u8 = 0;
    let mut draws: u8 = 0;
    let mut defeats: u8 = 0;
    let mut invalids: u8 = 0;
    let mut victories: u8 = 0;
    for (game, action, _, _) in walk::offline(&mut board, &mut greedy, memory.deref()) {
        if action != -1 {
            continue;
        }

        total += 1;
        if game.invalid {
            invalids += 1;
        } else {
            match game.winner {
                Tile::Robson => victories += 1,
                Tile::Janete => defeats += 1,
                Tile::Empty => draws += 1,
            }
        }
        if total == 100 {
            break;
        }
    }

    println!("");
    println!("{} TOTAL", total);
    println!("{}% victories", victories as f64 / total as f64 * 100.0);
    println!("{}% draws", draws as f64 / total as f64 * 100.0);
    println!("{}% defeats", defeats as f64 / total as f64 * 100.0);
    println!("{}% invalids", invalids as f64 / total as f64 * 100.0);
    println!("");

    let threshold = 0.9;
    let victory_rate = (victories as f64) / (total as f64);
    assert!(victory_rate > threshold);
}
