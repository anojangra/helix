use repo::schemas::Quote;
use std::collections::BTreeMap;
use strategies;
use strategies::Strategy;
use TradeSignal;
use Window;
use calc;

/// standard deviation F
///
///
pub fn call(
    strategy: Strategy,
    trade_signals: &mut BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) {
    let windows = strategies::make_window(quotes, strategy.param as usize);
    for w in windows {
        let signal = generator(&w);
        strategies::insert_signal(trade_signals, &w, &strategy, &signal);
    }
}

fn generator(window: &Window) -> i32 {
    let close_diffs: Vec<f32> = strategies::diff(&window.window, 1);
    // println!("close diffs: {:?}", close_diffs);
    let std_dev = calc::std_dev(close_diffs);
    // println!("stddev: {}", std_dev);
    let current_diff = window.current_diff();
    // println!("current_diff: {}", current_diff);
    if current_diff <= (-std_dev * 2.0) {
        return 1;
    }
    return 0;
}

#[test]
fn test_std_dev_f() {
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
            close: 99.00,
            volume: 1000.50,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528749808.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 94.00,
            volume: 1000.49,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528750809.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 93.00,
            volume: 1000.79,
        },
    ];
    let windows = strategies::make_window(&test_vec, 3);

    // x <= -2 sigma
    let first_window = &windows[0];
    // println!("first_window: {:?}", first_window);
    let signal = generator(&first_window);
    assert_eq!(1, signal);

    // x < -2 sigma
    let second_window = &windows[1];
    // println!("second_window: {:?}", second_window);
    let signal = generator(&second_window);
    assert_eq!(1, signal);

    let third_window = &windows[2];
    // println!("third window: {:?}", third_window);
    let signal = generator(&third_window);
    assert_eq!(0, signal);
}
