extern crate marskov;

use std::sync::Arc;
use std::ops::Deref;
use std::sync::RwLock;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};

use marskov::memories::Table;
use marskov::{Brain, Game, Memory, Policy};
use marskov::tasks::{play, train};
use marskov::policies::{Greedy, Random};

#[derive(Eq, Clone, Copy, Debug, Hash, PartialEq)]
enum Trade {
    Buy,
    Sell,
}

impl Default for Trade {
    fn default() -> Trade {
        Trade::Buy
    }
}

#[derive(Clone, Debug)]
struct Market {
    step: usize,
    ether: f64,
    bitcoin: f64,
    price: f64,
    last: f64,
    prices: Vec<f64>,
}

impl Market {
    fn bitcoin_total(&self) -> f64 {
        self.bitcoin + self.ether * self.price
    }

    fn is_final(&self) -> bool {
        self.step + 1 == self.prices.len()
    }

    fn reset(&mut self) {
        self.ether = 0.;
        self.bitcoin = 1.;
        self.step = 0;
        self.last = 1.;
        self.price = self.prices[0];
    }
}

impl Default for Market {
    fn default() -> Market {
        let step = 0;
        let prices = vec![
            0.0021652099999999999,
            0.0021769200000000002,
            0.0021503400000000002,
            0.0022436999999999999,
            0.0021806899999999999,
            0.0022000000000000001,
            0.002215,
            0.00204944,
            0.00217001,
            0.00219001,
            0.00223245,
            0.0023675300000000001,
            0.0026499000000000002,
            0.0025984900000000002,
            0.0027921999999999999,
            0.0033499699999999999,
            0.0031486399999999999,
            0.0034795099999999999,
            0.0036847099999999999,
            0.0036149999999999997,
            0.0036793400000000001,
            0.0037988700000000002,
            0.0039398599999999999,
            0.0050299999999999997,
            0.0052924299999999999,
            0.0063746699999999998,
        ];
        let price = prices[step];

        Market {
            step: step,
            ether: 0.,
            bitcoin: 1.,
            prices: prices,
            price: price,
            last: 1.,
        }
    }
}

impl Game<(usize, bool), Trade> for Market {
    fn state(&self) -> (usize, bool) {
        (self.step, self.bitcoin > 0.)
    }

    fn actions(&self) -> Vec<Trade> {
        vec![Trade::Buy, Trade::Sell]
    }

    fn act(&mut self, trade: &Trade) {
        if self.is_final() {
            self.reset();
            return;
        }

        self.last = self.bitcoin_total();

        if *trade == Trade::Sell {
            self.bitcoin += self.ether * self.price;
            self.ether = 0.;
        }
        if *trade == Trade::Buy {
            self.ether += self.bitcoin / self.price;
            self.bitcoin = 0.;
        }

        self.step += 1;
        self.price = self.prices[self.step];
    }

    fn reward(&self) -> f64 {
        if self.last < self.bitcoin_total() {
            return 0.1;
        }
        if self.last > self.bitcoin_total() {
            return -0.1;
        }
        0.0
        // println!("{} {} {}", 1. - self.bitcoin_total() / self.last, self.bitcoin_total(), self.last);
        // 1. - self.bitcoin_total() / self.last
    }
}

#[test]
fn solves_crypto() {
    let (sender, receiver) = channel();
    let table: Table<(usize, bool), Trade> = Table::default();
    let memory = Arc::new(RwLock::new(table));

    let training_memory = memory.clone();
    spawn(move || {
        train(
            &Market::default(),
            &mut Greedy::default(),
            &training_memory,
            &receiver,
            &Brain::new(1.0, 0.0),
        )
    });

    let playing_memory = memory.clone();
    spawn(move || {
        play(
            &mut Market::default(),
            &mut Random::default(),
            playing_memory.deref(),
            &sender,
        )
    });

    sleep(Duration::from_millis(100));

    let memory = memory.read().unwrap();
    let mut greedy = Greedy::default();
    let mut market = Market::default();

    println!("");
    loop {
        let mut action_values = vec![];
        let state = market.state();

        for action in market.actions() {
            let value = memory.get(&state, &action);
            action_values.push((action, value));
        }

        let action = greedy.choose(action_values).unwrap();

        market.act(&action);
        println!("{:?} {} {}", action, market.bitcoin_total(), market.price);

        if market.is_final() {
            break;
        }
    }

    println!("PROFIT {}", market.bitcoin_total() - 1.);
    assert_eq!(market.bitcoin_total(), 3.66542006775799);
}
