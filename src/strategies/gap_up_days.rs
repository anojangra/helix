use schemas::Quote;
use std::collections::BTreeMap;
use strategies;
use strategies::Strategy;
use trade_signal::TradeSignal;

/// Gap up days
/// 
/// Calculates gap up days
/// A gap up is identified when the Open is higher than the previous close
/// The signal returns 1 when the number of consecutive up days equals param
/// 
pub fn call(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    let windows = strategies::window(quotes, strategy.param as usize);
    for w in windows {
        let signal = gap_up_days(&w, strategy.param);
        strategies::insert_signal(trade_signals, &w, &strategy, &signal);
    }
}

fn gap_up_days(window: &strategies::Window, param: i32) -> i32 {
    let mut gap_up_days: Vec<i32> = vec![];
    let quotes = strategies::flatten_window(window);
    for i in 1..quotes.len() {
        let current_quote = &quotes[i];
        let previous_quote = &quotes[i - 1];
        if current_quote.open > previous_quote.close {
            gap_up_days.push(1);
        } else {
            gap_up_days.push(0);
        }
    }
    let sum_signals: i32 = gap_up_days.iter().sum();
    if sum_signals == param {
        return 1;
    } else {
        return 0;
    }
}

#[test]
fn test_gapupdays() {
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
            open: 99.00,
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
            close: 101.00,
            volume: 999.75,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528748807.0,
            open: 102.00,
            high: 105.00,
            low: 99.00,
            close: 102.00,
            volume: 1000.50,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528749808.0,
            open: 103.00,
            high: 105.00,
            low: 99.00,
            close: 103.00,
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
    // println!("first_window: {:?}", first_window);
    let signal = gap_up_days(&first_window, 3);
    assert_eq!(0, signal);
    let second_window = &windows[1];
    // println!("second_window: {:?}", second_window);
    let signal = gap_up_days(&second_window, 3);
    assert_eq!(1, signal);
}
