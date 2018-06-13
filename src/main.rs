//! # Helix is awesome
//!
//! Where does the rest of this go?
//!

extern crate postgres;
extern crate rand;
extern crate uuid;
extern crate chrono;

mod chromosome;
mod config;
mod controls;
mod dna;
mod repo;
mod schemas;
mod strategies;
mod trade_signal;
mod writer;

use chromosome::Chromosome;
use repo::get_quotes_by_symbol;
use repo::get_tickers;
use schemas::Quote;
use std::collections::BTreeMap;
use std::collections::HashMap;
use strategies::Strategy;
use trade_signal::TradeSignal;
// use writer;

fn main() {
    println!("Hello, world!");
    // init hash map of quotes
    // key: ticker
    // value: vec of quote
    // this takes 5 seconds
    let quotes_repo = init_quotes_repo();
    let dnas = dna::generate_dnas(12, 10000);
    for i in 1..2 {
        println!("Running generation: {}", i);
        let mut chromosomes: Vec<Chromosome> = vec![];
        if i == 1 {
            chromosomes = controls::generate_chromosomes(dnas.clone(), i, config::TARGET_TICKER)
        }
        writer::write_chromosomes::call(&chromosomes);
        // for chromosome in chromosomes {
        //     // println!("{:?}", chromosome);
        //     // Takes 9 seconds
        //     repo::insert_chromosome::call(chromosome);
        // }
    }
    // generate_signals(&chromosome, &quotes_repo);
    println!("quotes repo has {} keys", quotes_repo.len());
}

/// Initializes hashmap for quotes
///
fn init_quotes_repo() -> HashMap<String, Vec<Quote>> {
    let mut repo = HashMap::new();
    for ticker in get_tickers::call() {
        println!("{:?}", ticker);
        let quotes = get_quotes_by_symbol::call(&ticker.symbol);
        repo.insert(ticker.symbol, quotes);
    }
    repo
}

/// Run strategy
///
/// Splits chromosome to strategies
///
fn generate_signals(chromosome: &Chromosome, quotes_repo: &HashMap<String, Vec<Quote>>) {
    let mut trade_signals: BTreeMap<String, TradeSignal> = BTreeMap::new();
    let strategies = strategies::expand_strategies(chromosome);
    for strategy in strategies {
        trade_signals = match quotes_repo.get(&strategy.ticker) {
            Some(quotes) => generate_strategy_signals(strategy, trade_signals, quotes),
            None => panic!("this is a terrible mistake!"),
        };
    }

    println!("writing to disk");
    writer::write_signals::call(trade_signals, chromosome)
    // for signal in trade_signals {
    //     if signal.1.signals[0] == 1 {
    //         println!("{:?}", signal);
    //     }
    // }
}

/// Generate strategy signals
///
fn generate_strategy_signals(
    strategy: Strategy,
    trade_signals: BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) -> BTreeMap<String, TradeSignal> {
    println!("quotes len: {}  in run strategy", quotes.len());
    let updated_trade_signals = match strategy.code.as_ref() {
        "llv" => strategies::lowest_low_value::call(strategy, trade_signals, quotes),
        "hhv" => strategies::highest_high_value::call(strategy, trade_signals, quotes),
        _ => panic!("No such strategy"),
    };
    updated_trade_signals
}

#[test]
fn test_for_loop() {
    for i in 1..4 {
        println!("{}", i)
    }
}
