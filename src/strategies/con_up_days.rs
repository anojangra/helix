use strategies::Strategy;
use std::collections::BTreeMap;
use trade_signal::TradeSignal;
use schemas::Quote;
use strategies;

pub fn call(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    let windows = strategies::window(quotes, strategy.param as usize);
    for w in windows {
        let signal = con_up_days(&w);
        strategies::insert_signal(trade_signals, &w, &strategy, &signal);
    }
}

// fn up_days(
//     strategy: &Strategy,
//     trade_signals: &mut BTreeMap<String, TradeSignal>,
//     quotes: &Vec<Quote>,
// ) {
//     let lags = strategies::lag(quotes, 1);
// }
fn con_up_days(window: &strategies::Window) -> i32 {
    let mut up_days: Vec<i32> = vec![];
    let mut w = window.window.clone();
    w.push(window.current_quote.clone());
    println!("w window: {:?}", w);
    0
}

#[test]
fn test_conupdays() {
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
    let windows = strategies::window(&test_vec, 3);
    let first_window = &windows[0];
    println!("first_window: {:?}", first_window);
    let signal = con_up_days(&first_window);
    assert_eq!(0, signal);
}
