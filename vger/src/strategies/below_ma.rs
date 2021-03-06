use repo::schemas::Quote;
use std::collections::BTreeMap;
use strategies;
use strategies::Strategy;
use TradeSignal;
use calc;

/// Below Moving Average
/// 
/// 
pub fn call(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    let windows = strategies::make_window(quotes, strategy.param as usize);
    for w in windows {
        let signal = below_ma(&w);
        strategies::insert_signal(trade_signals, &w, &strategy, &signal);
    }
}

fn below_ma(window: &strategies::Window) -> i32 {
    let closes: Vec<f32> = window.window.iter().map(|quote| quote.close).collect();
    let ma = calc::average(closes);
    if window.current_quote.close < ma {
        return 1;
    }
    return 0;
}

#[test]
fn test_below_ma() {
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
            close: 101.00,
            volume: 999.75,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528748807.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 102.00,
            volume: 1000.50,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528749808.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 100.00,
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
    let windows = strategies::make_window(&test_vec, 3);
    let first_window = &windows[0];
    // println!("first_window: {:?}", first_window);
    let signal = below_ma(&first_window);
    assert_eq!(0, signal);
    let second_window = &windows[1];
    // println!("second_window: {:?}", first_window);
    let signal = below_ma(&second_window);
    assert_eq!(1, signal);
}
