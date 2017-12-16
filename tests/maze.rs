extern crate marskov;

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};

use marskov::game::Game;
use marskov::brain::Brain;
use marskov::policy::Policy;
use marskov::memory::Memory;
use marskov::tasks::play::play;
use marskov::tasks::train::train;
use marskov::memories::table::Table;
use marskov::policies::greedy::Greedy;
use marskov::policies::random::Random;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Move {
    fn default() -> Move {
        Move::Left
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Love,
    Death,
    Empty,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Maze {
    pub current: (i8, i8),
    pub tiles: [[Tile; 3]; 3],
}

impl Maze {
    fn current_tile(&self) -> Tile {
        let (r0, c0) = self.current;
        self.tiles[r0 as usize][c0 as usize]
    }
}

impl Default for Maze {
    fn default() -> Maze {
        Maze {
            current: (0, 0),
            tiles: [
                [Tile::Empty, Tile::Death, Tile::Love],
                [Tile::Empty, Tile::Death, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty],
            ],
        }
    }
}

impl Game<Move> for Maze {
    fn actions(&self) -> Vec<Move> {
        vec![Move::Left, Move::Right, Move::Up, Move::Down]
    }
    fn act(&mut self, m: &Move) {
        let (r0, c0) = self.current;

        let r1 = r0 + match *m {
            Move::Up => -1,
            Move::Down => 1,
            _ => 0,
        };

        let c1 = c0 + match *m {
            Move::Left => -1,
            Move::Right => 1,
            _ => 0,
        };

        if self.current_tile() != Tile::Empty {
            self.current = (0, 0);
        } else if r1 > -1 && r1 < 3 && c1 > -1 && c1 < 3 {
            self.current = (r1, c1);
        }
    }
    fn reward(&self) -> f64 {
        match self.current_tile() {
            Tile::Death => -1.0,
            Tile::Love => 1.0,
            _ => -0.1,
        }
    }
}

#[test]
fn solves_maze() {
    let (sender, receiver) = channel();
    let table: Table<Maze, Move> = Table::default();
    let memory = Arc::new(RwLock::new(table));

    let training_memory = memory.clone();
    spawn(|| {
        train(
            Maze::default(),
            Greedy::default(),
            training_memory,
            receiver,
            Brain::new(0.5, 0.5),
        )
    });

    let playing_memory = memory.clone();
    spawn(|| play(Maze::default(), Random::default(), playing_memory, sender));

    sleep(Duration::from_millis(1000));

    let memory = memory.read().unwrap();
    let mut greedy = Greedy::default();
    let mut maze = Maze::default();

    let path = [(1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2)];

    for position in path.iter() {
        let mut avs = vec![];
        for action in maze.actions() {
            let value = memory.get(&maze, &action);
            avs.push((action, value));
        }
        maze.act(greedy.choose(&avs).unwrap());

        assert_eq!(maze.current, *position);
    }
}
