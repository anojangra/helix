use repo::schemas::Quote;
use std::collections::BTreeMap;
use strategies;
use strategies::insert_signal;
use strategies::Strategy;
use trade_signal::TradeSignal;
use window::Window;

pub fn call(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    let windows = strategies::make_window(quotes, strategy.param as usize);
    for w in windows {
        let signal = highest_high_value(&w);
        insert_signal(trade_signals, &w, &strategy, &signal);
    }
}

/// Calculate highest high in window
///
/// Triggers a signal is close exceeds highest high in window period
///
fn highest_high_value(window: &Window) -> i32 {
    let high_values: Vec<f32> = window.window.iter().map(|quote| quote.high).collect();
    let highest_value = high_values.iter().fold(0_f32, |acc, x| acc.max(*x));
    if window.current_quote.close > highest_value {
        return 1;
    }
    return 0;
}

#[test]
fn test_hhv() {
    let test_vec = vec![
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528745804.0,
            open: 110.00,
            high: 112.00,
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
            close: 200.00,
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
    let windows = strategies::make_window(&test_vec, 3);
    let first_window = &windows[0];
    let signal = highest_high_value(&first_window);
    assert_eq!(1, signal);
}
