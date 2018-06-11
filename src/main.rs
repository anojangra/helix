//! # Helix is awesome
//!
//! Where does the rest of this go?
//!

extern crate postgres;
extern crate uuid;

mod chromosome;
mod repo;
mod schemas;
mod strategies;
mod trade_signal;

use chromosome::Chromosome;
use repo::get_quotes_by_symbol;
use repo::get_tickers;
use schemas::Quote;
use std::collections::BTreeMap;
use std::collections::HashMap;
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

    for signal in trade_signals {
        println!("{:?}", signal);
    }
}

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

