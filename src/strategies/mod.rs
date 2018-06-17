use chromosome::Chromosome;
use schemas::Quote;
use std::collections::BTreeMap;
use trade_signal;
use trade_signal::TradeSignal;
use uuid::Uuid;

pub mod con_up_days;
pub mod highest_high_value;
pub mod lowest_low_value;

#[derive(Debug, Clone)]
pub struct Strategy {
    pub chromosome_id: Uuid,
    pub strategy: String,
    pub code: String,
    pub ticker: String,
    pub param: i32,
    pub target_ticker: String,
    pub generation: i32,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub window: Vec<Quote>,
    pub current_quote: Quote,
}

#[derive(Debug, Clone)]
pub struct Lag {
    pub lag_quote: Quote,
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
pub fn expand_strategies(chromosome: Chromosome) -> Vec<Strategy> {
    let strategies: Vec<&str> = chromosome.chromosome.split("::").collect();
    let expanded_strategies = strategies
        .into_iter()
        .map(|s| expand_strategy(chromosome.clone(), s.to_string()))
        .collect();
    expanded_strategies
}

/// Expands chrosomes to Strategy
pub fn expand_strategy(chromosome: Chromosome, strategy: String) -> Strategy {
    let v: Vec<&str> = strategy.split(":").collect();
    let strategy_name = strategy.clone();
    debug!("{:?}", strategy_name);
    Strategy {
        chromosome_id: chromosome.id,
        strategy: strategy_name,
        code: v[0].to_string(),
        ticker: v[1].to_string(),
        param: v[2].parse::<i32>().unwrap(),
        target_ticker: chromosome.target_ticker.clone(),
        generation: chromosome.generation,
    }
}

/// Inserts a new, empty signal if the signal does not exist
fn insert_signal(
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    window: &Window,
    strategy: &Strategy,
    signal: &i32,
) {
    let ts_string = window.current_quote.ts.to_string();
    let trade_signal = match trade_signals.get(&ts_string) {
        Some(s) => update_signal(s, strategy, signal),
        None => trade_signal::init_trade_signal(strategy, &window, signal),
    };
    trade_signals.insert(ts_string, trade_signal);
}

/// Updates existing signal in btreemap
fn update_signal(trade_signal: &TradeSignal, strategy: &Strategy, signal: &i32) -> TradeSignal {
    let mut strategies = trade_signal.strategies.clone();
    strategies.push(strategy.strategy.clone());
    let mut signals = trade_signal.signals.clone();
    signals.push(signal.clone());
    TradeSignal {
        chromosome_id: trade_signal.chromosome_id,
        ts: trade_signal.ts,
        strategies: strategies,
        signals: signals,
        target_ticker: trade_signal.target_ticker.clone(),
        hard_signal: trade_signal.hard_signal,
        generation: trade_signal.generation,
        pnl: 0.0,
        ret: 0.0,
    }
}

/// Cast windows from list of quotes
///
/// A window is x quotes prior to the current quote
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

// Takes the lagged window of quotes and the current window and creates a
// a single vector of quote
fn flatten_window(window: &Window) -> Vec<Quote> {
    let mut w = window.window.clone();
    w.push(window.current_quote.clone());
    w
}

fn lag(quotes: &Vec<Quote>, periods: usize) -> Vec<Lag> {
    let mut lag: Vec<Lag> = vec![];
    for n in periods..quotes.len() {
        let lag_n = n - periods;
        let lag_quote = &quotes[lag_n];
        let new_lag = Lag {
            lag_quote: lag_quote.clone(),
            current_quote: quotes[n].clone(),
        };
        lag.push(new_lag);
    }
    lag
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
        chromosome_length: 2,
        kelly: 0.0,
        cum_pnl: 0.0,
        variance: 0.0,
        mean_return: 0.0,
        w_kelly: 0.0,
        num_of_trades: 0,
        rank: 0,
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

    let actual = expand_strategy(chromosome, "llv:krakenUSD:2".to_string());

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
            ts: 1528746805.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.80,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528747806.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 999.75,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528748807.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.50,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528749808.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.49,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528750809.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 99.00,
            volume: 1000.79,
        },
    ];
    let windows = window(&test_vec, 3);
    let first_window = &windows[0];
    assert_eq!(first_window.window[0].ts, 1528745804.0);
    assert_eq!(first_window.current_quote.ts, 1528748807.0);
}

#[test]
fn test_flatten_window() {
    let w = Window {
        window: vec![
            Quote {
                ticker: "AAPL".to_string(),
                ts: 1528745804.0,
                open: 100.0,
                high: 105.0,
                low: 99.0,
                close: 99.0,
                volume: 1000.2,
            },
            Quote {
                ticker: "AAPL".to_string(),
                ts: 1528746805.0,
                open: 100.0,
                high: 105.0,
                low: 99.0,
                close: 99.0,
                volume: 1000.8,
            },
            Quote {
                ticker: "AAPL".to_string(),
                ts: 1528747806.0,
                open: 100.0,
                high: 105.0,
                low: 99.0,
                close: 99.0,
                volume: 999.75,
            },
        ],
        current_quote: Quote {
            ticker: "AAPL".to_string(),
            ts: 1528748807.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 99.0,
            volume: 1000.5,
        },
    };
    let f_window = flatten_window(&w);
    let expected = vec![
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528745804.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 99.0,
            volume: 1000.2,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528746805.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 99.0,
            volume: 1000.8,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528747806.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 99.0,
            volume: 999.75,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528748807.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 99.0,
            volume: 1000.5,
        },
    ];
    assert_eq!(expected[0].ts, f_window[0].ts);
    assert_eq!(expected[1].ts, f_window[1].ts);
    assert_eq!(expected[2].ts, f_window[2].ts);
    assert_eq!(expected[3].ts, f_window[3].ts);

}
