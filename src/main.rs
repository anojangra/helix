//! # Helix is awesome
//!
//! Where does the rest of this go?
//!

extern crate chrono;
extern crate postgres;
extern crate rand;
extern crate uuid;

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
use schemas::Return;
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
    let dnas = dna::generate_dnas(12, 5);
    let returns = init_returns();
    for i in 1..2 {
        println!("Running generation: {}", i);
        let mut chromosomes: Vec<Chromosome> = vec![];
        if i == 1 {
            chromosomes = controls::generate_chromosomes(dnas.clone(), i, config::TARGET_TICKER)
        }
        // writer::write_chromosomes::call(&chromosomes);
        // repo::copy_chromosomes::call();
        let local = chromosomes.clone();
        for (i, chromosome) in local.iter().enumerate() {
            // println!("processing chromosome: {:?}", chromosome);
            let mut trade_signals = generate_signals(chromosome, &quotes_repo);
            // Merge returns
            merge_returns(&mut trade_signals, &returns);
            // Calc hard trade signal
            calc_pnl(&mut trade_signals, chromosome);
            // Write signals to disk
            println!("writing to disk");
            writer::write_signals::call(&trade_signals);
            // Filter trade signals with hard signals
            let signaled_trades: Vec<TradeSignal> = trade_signals
                .into_iter()
                .map(|x| x.1)
                .filter(|signal| signal.hard_signal == 1)
                .collect();

            // Calculate pnl
            let cum_pnl: f32 = signaled_trades.iter().map(|x| x.pnl).sum();
            let mean_return = mean_return(&signaled_trades);
            let variance = variance(&signaled_trades);
            let kelly = kelly(mean_return, variance);
            let num_of_trades = signaled_trades.len() as i32;

            let updated_chromosome = Chromosome {
                id: chromosome.id,
                chromosome: chromosome.chromosome.clone(),
                chromosome_length: chromosome.chromosome_length,
                dna: chromosome.dna.clone(),
                generation: chromosome.generation,
                target_ticker: chromosome.target_ticker.clone(),
                cum_pnl: cum_pnl,
                mean_return: mean_return,
                variance: variance,
                kelly: kelly,
                num_of_trades: num_of_trades,
                w_kelly: kelly * num_of_trades as f32,
                rank: chromosome.rank,
            };
            // Update chromosomes
            chromosomes[i] = updated_chromosome;
        }
        chromosomes.sort_by_key(|c| c.w_kelly.to_string());
        for chromosome in chromosomes {
            println!{"{:?}", chromosome};
        }
    }

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

/// Initializes Btreemap for returns
///
fn init_returns() -> BTreeMap<String, Return> {
    let mut repo: BTreeMap<String, Return> = BTreeMap::new();
    for ret in repo::get_returns::call(config::TARGET_TICKER.to_string()) {
        let ts = ret.ts.to_string();
        repo.insert(ts, ret);
    }
    repo
}

/// Run strategy
///
/// Splits chromosome to strategies
///
fn generate_signals(
    chromosome: &Chromosome,
    quotes_repo: &HashMap<String, Vec<Quote>>,
) -> BTreeMap<String, TradeSignal> {
    let mut trade_signals: BTreeMap<String, TradeSignal> = BTreeMap::new();
    let strategies = strategies::expand_strategies(chromosome);
    for strategy in strategies {
        match quotes_repo.get(&strategy.ticker) {
            Some(quotes) => {
                generate_strategy_signals(strategy, &mut trade_signals, quotes);
            }
            None => panic!("this is a terrible mistake!"),
        };
    }
    trade_signals
}

/// Merge returns into trade signals
fn merge_returns(
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    returns: &BTreeMap<String, Return>,
) {
    let local = trade_signals.clone();
    for key_value in &local {
        let ts_string = key_value.1.ts.to_string();
        match local.get(&ts_string) {
            Some(s) => update_merge_trade_signal(s, trade_signals, &returns, &ts_string),
            None => (),
        };
    }
}

fn update_merge_trade_signal(
    trade_signal: &TradeSignal,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    returns: &BTreeMap<String, Return>,
    ts_string: &String,
) {
    match returns.get(ts_string) {
        Some(ret) => {
            let t = trade_signal.clone();
            let updated = TradeSignal { ret: ret.ret, ..t };
            trade_signals.insert(ts_string.clone(), updated);
        }
        None => (),
    };
}

// Calculate hard signal and pnl
fn calc_pnl(trade_signals: &mut BTreeMap<String, TradeSignal>, chromosome: &Chromosome) {
    let local = trade_signals.clone();
    for trade_signal in &local {
        let mut s = trade_signal.1.clone();
        let agg_signal: i32 = s.signals.iter().sum();
        if chromosome.chromosome_length == agg_signal {
            s.hard_signal = 1;
            s.pnl = s.ret * 1.0;
        }

        trade_signals.insert(trade_signal.0.clone(), s);
    }
}

// Calculate mean
fn mean_return(signaled_trades: &Vec<TradeSignal>) -> f32 {
    let cum_pnl: f32 = signaled_trades.iter().map(|x| x.pnl).sum();
    if signaled_trades.len() > 0 {
        let mean_return: f32 = cum_pnl / signaled_trades.len() as f32;
        return mean_return;
    };
    return 0.0 as f32;
}

// Calculate variance
fn variance(signaled_trades: &Vec<TradeSignal>) -> f32 {
    if signaled_trades.len() > 0 {
        let mean = mean_return(&signaled_trades);
        let diffs: Vec<f32> = signaled_trades
            .iter()
            .map(|x| (x.pnl - mean).powi(2) as f32)
            .collect();
        let sum_diffs: f32 = diffs.iter().sum();
        let v: f32 = sum_diffs / signaled_trades.len() as f32;
        return v;
    }
    0.0
}

/// Calculate kelly
fn kelly(mean: f32, variance: f32) -> f32 {
    if variance > 0.0 {
        return mean / variance;
    }
    return 0.0;
}

/// Generate strategy signals
///
fn generate_strategy_signals(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    match strategy.code.as_ref() {
        "llv" => strategies::lowest_low_value::call(strategy, trade_signals, quotes),
        "hhv" => strategies::highest_high_value::call(strategy, trade_signals, quotes),
        _ => panic!("No such strategy"),
    };
}

#[test]
fn test_for_loop() {
    for i in 1..4 {
        println!("{}", i)
    }
}
