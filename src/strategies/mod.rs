use chromosome::Chromosome;
use schemas::Quote;
use std::collections::BTreeMap;
use trade_signal;
use trade_signal::TradeSignal;
use uuid::Uuid;

pub mod highest_high_value;
pub mod lowest_low_value;

#[derive(Debug, Clone)]
pub struct Strategy {
    pub chromosome_id: Uuid,
    pub strategy: String,
    pub code: String,
    pub ticker: String,
    pub param: i64,
    pub target_ticker: String,
    pub generation: i32,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub window: Vec<Quote>,
    pub current_quote: Quote,
}

/// Expands chromosome of strategies to a list of strategies
///
/// "llv:AAPL:2::gapupday:GOOG:10"
///
/// Returns
/// [
///     Strategy {
///         name: "llv",
///         ticker: "AAPL",
///         param: 2
///     },
///     Strategy {
///         name: "gapupday",
///         ticker: "GOOG",
///         param: 10
///     }
/// ]
///
pub fn expand_strategies(chromosome: &Chromosome) -> Vec<Strategy> {
    let strategies: Vec<&str> = chromosome.chromosome.split("::").collect();
    let expanded_strategies = strategies
        .into_iter()
        .map(|s| expand_strategy(chromosome, s.to_string()))
        .collect();
    expanded_strategies
}

/// Expands chrosomes to Strategy
/// 
pub fn expand_strategy(chromosome: &Chromosome, strategy: String) -> Strategy {
    let v: Vec<&str> = strategy.split(":").collect();
    let strategy_name = strategy.clone();
    Strategy {
        chromosome_id: chromosome.id,
        strategy: strategy_name,
        code: v[0].to_string(),
        ticker: v[1].to_string(),
        param: v[2].parse::<i64>().unwrap(),
        target_ticker: chromosome.target_ticker.clone(),
        generation: chromosome.generation,
    }
}

/// Inserts a new, empty signal if the signal does not exist
///
fn insert_signal(
    trade_signals: BTreeMap<String, TradeSignal>,
    window: Window,
    strategy: Strategy,
    signal: i32,
) -> BTreeMap<String, TradeSignal> {
    let mut signals = trade_signals;
    let ts_string = window.current_quote.ts.to_string();
    let trade_signal = match signals.get(&ts_string) {
        Some(s) => update_signal(s, strategy, signal),
        None => trade_signal::init_trade_signal(strategy, window, signal),
    };
    signals.insert(ts_string, trade_signal);
    signals
}

/// Updates existing signal in btreemap
///
fn update_signal(trade_signal: &TradeSignal, strategy: Strategy, signal: i32) -> TradeSignal {
    let mut strategies = trade_signal.strategies.clone();
    strategies.push(strategy.strategy);
    let mut signals = trade_signal.signals.clone();
    signals.push(signal);
    TradeSignal {
        chromosome_id: trade_signal.chromosome_id,
        ts: trade_signal.ts,
        strategies: strategies,
        signals: signals,
        target_ticker: trade_signal.target_ticker.clone(),
        hard_signal: trade_signal.hard_signal,
        generation: trade_signal.generation,
    }
}

/// Cast windows from list of quotes
///
/// returns tuple: (array of windows, current_quote)
///
fn window(quotes: &Vec<Quote>, length: usize) -> Vec<Window> {
    let mut windows: Vec<Window> = vec![];
    for n in length..quotes.len() {
        let start_index = n - length;
        let window = quotes[start_index..n].to_vec();
        let new_window = Window {
            window: window,
            current_quote: quotes[n].clone(),
        };
        windows.push(new_window);
    }
    windows
}

#[cfg(test)]
#[test]
fn test_expand_strategy() {
    let chromosome = Chromosome {
        id: Uuid::new_v4(),
        chromosome: "llv:krakenUSD:2::hhv:coinbaseUSD:3".to_string(),
        target_ticker: "xlf".to_string(),
        dna: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        generation: 1,
    };

    let expected = Strategy {
        chromosome_id: chromosome.id,
        strategy: "llv:krakenUSD:2".to_string(),
        code: String::from("llv"),
        ticker: String::from("krakenUSD"),
        target_ticker: chromosome.target_ticker.clone(),
        param: 2,
        generation: chromosome.generation,
    };

    let actual = expand_strategy(&chromosome, "llv:krakenUSD:2".to_string());

    assert_eq!(expected.code, actual.code);
    assert_eq!(expected.ticker, actual.ticker);
    assert_eq!(expected.param, actual.param);
}

#[test]
fn test_window() {
    let test_vec = vec![
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528745804.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.20,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528746804.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.80,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528747804.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 999.75,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528748804.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.50,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528749804.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.49,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528750804.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.79,
        },
    ];
    let windows = window(&test_vec, 3);
    let first_quote = &windows[0].window[0];
    assert_eq!(first_quote.ticker, "AAPL".to_string());
}

// #[test]
// fn test_expand_strategies() {
//     let actual = expand_strategies("llv:krakenUSD:2::hhv:krakenUSD:5".to_string());

//     assert_eq!("llv".to_string(), actual[0].name);
//     assert_eq!("krakenUSD".to_string(), actual[0].ticker);
//     assert_eq!(2, actual[0].param);
// }
