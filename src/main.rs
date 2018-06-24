//! # Helix is awesome
//!
//! Where does the rest of this go?
//!
#[macro_use]
extern crate log;
extern crate chrono;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate postgres;
extern crate rand;
extern crate threadpool;
extern crate uuid;

mod chromosome;
mod config;
mod darwin;
mod dna;
mod repo;
mod schemas;
mod strategies;
mod trade_signal;
mod writer;
mod window;

use chromosome::Chromosome;
use repo::get_quotes_by_symbol;
use repo::get_tickers;
use schemas::Quote;
use schemas::Return;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use strategies::Strategy;
use trade_signal::TradeSignal;

fn main() {
    env_logger::init();
    info!("Hello, world!");
    let quotes_repo = init_quotes_repo();
    let dnas = dna::generate_dnas(12, config::POPULATION_SIZE);
    let returns = init_returns();
    repo::init::init_trade_signals();
    repo::init::init_chromosomes();
    let mut ranked_chromosomes: Vec<Chromosome> = vec![];
    for i in 1..4 {
        warn!("Running generation: {}", i);
        let mut chromosomes: Vec<Chromosome> = vec![];
        if i == 1 {
            chromosomes = chromosome::generate_chromosomes(dnas.clone(), i, config::TARGET_TICKER)
        } else {
            chromosomes = darwin::evolve(ranked_chromosomes, i);
        }
        let c_len = *&chromosomes.len();
        let (c_tx, c_rx) = channel();
        let (throttle_tx, throttle_rx) = crossbeam_channel::bounded(8);
        for chromosome in chromosomes {
            let q_clone = quotes_repo.clone();
            let r_clone = returns.clone();
            let tx_n = c_tx.clone();
            let t_rx = throttle_rx.clone();
            throttle_tx.send(1);
            debug!("Throttle length: {}", throttle_rx.len());
            thread::spawn(move || {
                tx_n.send(process_chromosome(&chromosome, q_clone, r_clone))
                    .unwrap();
                t_rx.recv().unwrap();
            });
        }
        let updated_chromosomes: Vec<Chromosome> = c_rx.iter().take(c_len).map(|c| c).collect();
        ranked_chromosomes = rank_chromosomes(updated_chromosomes);
        writer::write_chromosomes::call(&ranked_chromosomes);
    }
    info!("So long and thanks for all the fish!");
}

/// Initializes hashmap for quotes
fn init_quotes_repo() -> HashMap<String, Vec<Quote>> {
    let mut repo = HashMap::new();
    for ticker in get_tickers::call() {
        debug!("{:?}", ticker);
        let quotes = get_quotes_by_symbol::call(&ticker.symbol);
        repo.insert(ticker.symbol, quotes);
    }
    repo
}

/// Initializes Btreemap for returns
fn init_returns() -> BTreeMap<String, Return> {
    debug!("Initializing returns");
    let mut repo: BTreeMap<String, Return> = BTreeMap::new();
    for ret in repo::get_returns::call(config::TARGET_TICKER.to_string()) {
        let ts = ret.ts.to_string();
        repo.insert(ts, ret);
    }
    repo
}

// Generate signals and metadata for chromosome
fn process_chromosome(
    chromosome: &Chromosome,
    quotes_repo: HashMap<String, Vec<Quote>>,
    returns: BTreeMap<String, Return>,
) -> Chromosome {
    let mut trade_signals = generate_signals(&chromosome, quotes_repo);
    merge_returns(&mut trade_signals, &returns);
    calc_pnl(&mut trade_signals, chromosome.clone());
    writer::write_signals::call(&trade_signals, &chromosome);
    update_chromosome(chromosome.clone(), trade_signals)
}

/// Run strategy
///
/// Splits chromosome to strategies
///
fn generate_signals(
    chromosome: &Chromosome,
    quotes_repo: HashMap<String, Vec<Quote>>,
) -> BTreeMap<String, TradeSignal> {
    let mut trade_signals: BTreeMap<String, TradeSignal> = BTreeMap::new();
    let strategies = strategies::expand_strategies(chromosome.clone());
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

/// Update merge trade signal
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
fn calc_pnl(trade_signals: &mut BTreeMap<String, TradeSignal>, chromosome: Chromosome) {
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
fn generate_strategy_signals(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    match strategy.code.as_ref() {
        "llv" => strategies::lowest_low_value::call(strategy, trade_signals, quotes),
        "hhv" => strategies::highest_high_value::call(strategy, trade_signals, quotes),
        "conupdays" => strategies::con_up_days::call(strategy, trade_signals, quotes),
        "condowndays" => strategies::con_down_days::call(strategy, trade_signals, quotes),
        "gapup" => strategies::gap_up_days::call(strategy, trade_signals, quotes),
        "gapdown" => strategies::gap_down_days::call(strategy, trade_signals, quotes),
        "belowma" => strategies::below_ma::call(strategy, trade_signals, quotes),
        "abovema" => strategies::above_ma::call(strategy, trade_signals, quotes),
        "stdeva" => strategies::stddev_a::call(strategy, trade_signals, quotes),
        "stdevb" => strategies::stddev_b::call(strategy, trade_signals, quotes),
        "stdevd" => strategies::stddev_d::call(strategy, trade_signals, quotes),
        "stdevf" => strategies::stddev_f::call(strategy, trade_signals, quotes),
        _ => panic!("No such strategy"),
    };
}

// Update chromsome with summary data
fn update_chromosome(
    chromosome: Chromosome,
    trade_signals: BTreeMap<String, TradeSignal>,
) -> Chromosome {
    let mut updated_chromosome = chromosome.clone();
    let total_trade_signals = &trade_signals.len();
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
    // Update chromosome
    updated_chromosome.cum_pnl = cum_pnl;
    updated_chromosome.mean_return = mean_return;
    updated_chromosome.variance = variance;
    updated_chromosome.kelly = kelly;
    updated_chromosome.num_of_trades = num_of_trades;
    updated_chromosome.w_kelly = kelly * (num_of_trades as f32 / *total_trade_signals as f32);
    updated_chromosome
}

// Rank chromosomes by w_kelly
fn rank_chromosomes(updated_chromosomes: Vec<Chromosome>) -> Vec<Chromosome> {
    let mut filtered_chromosomes: Vec<Chromosome> = updated_chromosomes
        .into_iter()
        .filter(|c| c.num_of_trades > 30)
        .collect();
    filtered_chromosomes.sort_by_key(|c| c.w_kelly as i32);
    let end_idx = filtered_chromosomes.len() as i32;
    let fittest = config::FITTEST as i32;
    let start_idx = end_idx - fittest;
    for i in start_idx..end_idx {
        let chromosome = &mut filtered_chromosomes[i as usize];
        let negative_rank = (end_idx - i - fittest - 1) as i32;
        chromosome.rank = negative_rank.abs() as i32;
    }
    filtered_chromosomes.clone()
}
