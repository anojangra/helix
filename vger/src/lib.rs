extern crate uuid;
#[macro_use]
extern crate log;
extern crate forge;
extern crate repo;

use repo::schemas::Quote;
use std::collections::BTreeMap;
use strategies::Strategy;
use uuid::Uuid;

pub mod strategies;

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
///
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

#[derive(Debug, Clone)]
pub struct Window {
    pub window: Vec<Quote>,
    pub current_quote: Quote,
}

impl Window {
    pub fn current_diff(&self) -> f32 {
        let end_idx = self.window.len() - 1;
        let end_window_quote = &self.window[end_idx.clone()];
        self.current_quote.close - end_window_quote.close
    }

    // Takes the lagged window of quotes and the current window and creates a
    // a single vector of quote
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
