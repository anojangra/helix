//! # Helix is awesome
//!
//! Where does the rest of this go?
//!

extern crate postgres;
extern crate rand;
extern crate uuid;

mod chromosome;
mod dna;
mod repo;
mod schemas;
mod strategies;
mod trade_signal;
mod config;

use chromosome::Chromosome;
use repo::get_quotes_by_symbol;
use repo::get_tickers;
use schemas::Quote;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use strategies::Strategy;
use trade_signal::TradeSignal;
use uuid::Uuid;

fn main() {
    println!("Hello, world!");
    // init hash map of quotes
    // key: ticker
    // value: vec of quote
    let quotes_repo = init_quotes_repo();

    let chromosome = Chromosome {
        id: Uuid::new_v4(),
        chromosome: "llv:krakenUSD:2::hhv:coinbaseUSD:3".to_string(),
        target_ticker: "xlf".to_string(),
        dna: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        generation: 1,
    };
    // Init btreemap of signals
    generate_signals(&chromosome, &quotes_repo);

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

    println!("wiriting to disk");
    write_signals(trade_signals, chromosome)
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

/// Write signals to disk
/// 
fn write_signals(signals: BTreeMap<String, TradeSignal>, chromosome: &Chromosome) {
    let mut f = File::create("/tmp/output.txt").expect("Unable to create file");
    for signal in signals {
        let s = signal.1;
        write!(
            f,
            "{},{},{}\n",
            s.chromosome_id,
            s.ts,
            fmt_vec_string(s.strategies)
        ).unwrap();
    }
}

/// Format vector of String
///
/// Formats the vector to be readable by postgresql as an array
///
fn fmt_vec_string(strings: Vec<String>) -> String {
    let mut strings = strings;
    let mut s = String::from("\"{");
    s.push_str(strings.remove(0).as_str());
    for string in strings {
        s.push_str(",");
        s.push_str(string.as_str());
    }
    let close_brace = "}\"";
    s.push_str(close_brace);
    s
}

#[test]
fn test_fmt_vec_string() {
    let t = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!("\"{A,B,C}\"", fmt_vec_string(t))
}
