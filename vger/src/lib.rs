//! Processes the chromosomes and generates signals from strategies
//!
extern crate uuid;
#[macro_use]
extern crate log;
extern crate forge;
extern crate repo;

use forge::Chromosome;
use repo::schemas::Quote;
use repo::schemas::Return;
use std::collections::BTreeMap;
use std::collections::HashMap;
use strategies::Strategy;
use uuid::Uuid;

pub mod strategies;
pub mod calc;

/// Struct for grouping daily trade signal data
#[derive(Debug, Clone)]
pub struct TradeSignal {
    pub chromosome_id: Uuid,
    pub ts: f64,
    pub strategies: Vec<String>,
    pub signals: Vec<i32>,
    pub target_ticker: String,
    pub hard_signal: i32,
    pub generation: i32,
    pub ret: f32,
    pub pnl: f32,
}

/// Initializes empty trade signal
pub fn init_trade_signal(strategy: &Strategy, window: &Window, signal: &i32) -> TradeSignal {
    let strategies = vec![strategy.strategy.clone()];
    let signals = vec![*signal];
    TradeSignal {
        chromosome_id: strategy.chromosome_id,
        ts: window.current_quote.ts,
        strategies: strategies,
        signals: signals,
        target_ticker: strategy.target_ticker.clone(),
        hard_signal: 0,
        generation: strategy.generation,
        ret: 0.0,
        pnl: 0.0,
    }
}

/// A window of `Quotes` of length n where `t^n < t^0` and the current quote at `t^0`
#[derive(Debug, Clone)]
pub struct Window {
    /// window of quotes
    pub window: Vec<Quote>,

    /// quote at `t^0`
    pub current_quote: Quote,
}

impl Window {
    /// Calculates diff of current close and previous close
    pub fn current_diff(&self) -> f32 {
        let end_idx = self.window.len() - 1;
        let end_window_quote = &self.window[end_idx.clone()];
        self.current_quote.close - end_window_quote.close
    }

    /// Takes the lagged window of quotes and the current window and creates a
    /// a single vector of quote
    pub fn flatten(&self) -> Vec<Quote> {
        let mut w = self.window.clone();
        w.push(self.current_quote.clone());
        w
    }
}

/// Generate strategy signals
pub fn generate_strategy_signals(
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

/// Generate signals from chromosome
pub fn generate_signals(
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
pub fn merge_returns(
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
pub fn update_merge_trade_signal(
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

/// Calculate hard signal and pnl
pub fn calc_pnl(trade_signals: &mut BTreeMap<String, TradeSignal>, chromosome: Chromosome) {
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

/// Calculate mean return from triggered trade signals
pub fn mean_return(signaled_trades: &Vec<TradeSignal>) -> f32 {
    let cum_pnl: f32 = signaled_trades.iter().map(|x| x.pnl).sum();
    if signaled_trades.len() > 0 {
        let mean_return: f32 = cum_pnl / signaled_trades.len() as f32;
        return mean_return;
    };
    return 0.0 as f32;
}

/// Calculates variance
pub fn variance(signaled_trades: &Vec<TradeSignal>) -> f32 {
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

/// Updates chromsome with summary data
pub fn update_chromosome(
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

    // Calculate summary data
    let cum_pnl: f32 = signaled_trades.iter().map(|x| x.pnl).sum();
    let mean_return = mean_return(&signaled_trades);
    let variance = variance(&signaled_trades);
    let kelly = calc::kelly(mean_return, variance);
    let num_of_trades = signaled_trades.len() as i32;
    let winning_trades: i32 = winning_trades(&signaled_trades);
    let losing_trades: i32 = losing_trades(&signaled_trades);
    let percentage_winners: f32 = percentage_winners(winning_trades, num_of_trades);

    // Update chromosome
    updated_chromosome.cum_pnl = cum_pnl;
    updated_chromosome.mean_return = mean_return;
    updated_chromosome.variance = variance;
    updated_chromosome.kelly = kelly;
    updated_chromosome.num_of_trades = num_of_trades;
    updated_chromosome.w_kelly = kelly * (num_of_trades as f32 / *total_trade_signals as f32);
    updated_chromosome.losing_trades = losing_trades;
    updated_chromosome.winning_trades = winning_trades;
    updated_chromosome.percentage_winners = percentage_winners;

    // println!("xxx chromosome: {:?}", updated_chromosome);
    updated_chromosome
}

/// Calculates winning trades
pub fn winning_trades(signaled_trades: &Vec<TradeSignal>) -> i32 {
    let winning_trades: Vec<&TradeSignal> = signaled_trades
        .iter()
        .filter(|signal| signal.pnl > 0.0)
        .collect();
    winning_trades.len() as i32
}

/// Calculates losing trades
pub fn losing_trades(signaled_trades: &Vec<TradeSignal>) -> i32 {
    let losing_trades: Vec<&TradeSignal> = signaled_trades
        .iter()
        .filter(|signal| signal.pnl < 0.0)
        .collect();
    losing_trades.len() as i32
}

/// Calculates percentage winners
///
/// Percentage winners is calculated as winners over total trades
pub fn percentage_winners(num_winners: i32, num_of_trades: i32) -> f32 {
    if num_of_trades < 0 {
        return 0.0;
    }
    return num_winners as f32 / num_of_trades as f32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
