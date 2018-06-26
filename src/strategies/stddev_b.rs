use repo::schemas::Quote;
use std::collections::BTreeMap;
use strategies;
use strategies::Strategy;
use trade_signal::TradeSignal;
use window::Window;

/// Standard deviation above 1 sigma and less than 2 sigmas
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
    let std_dev = strategies::std_dev(close_diffs);
    let current_diff = window.current_diff();
    if current_diff < (std_dev * 2.0) && current_diff >= std_dev {
        return 1;
    }
    return 0;
}

#[test]
fn test_std_dev_b() {
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
            close: 104.00,
            volume: 1000.49,
        },
        Quote {
            ticker: "AAPL".to_string(),
            ts: 1528750809.0,
            open: 100.00,
            high: 105.00,
            low: 99.00,
            close: 104.25,
            volume: 1000.79,
        },
    ];
    let windows = strategies::make_window(&test_vec, 3);
    
    // Test x = 1 sigma
    let first_window = &windows[0];
    // println!("first_window: {:?}", first_window);
    let signal = generator(&first_window);
    assert_eq!(1, signal);
    
    // Test x > 2 sigma
    let second_window = &windows[1];
    // println!("second_window: {:?}", second_window);
    let signal = generator(&second_window);
    assert_eq!(0, signal);


    let third_window = &windows[2];
    // println!("third window: {:?}", third_window);
    let signal = generator(&third_window);
    assert_eq!(0, signal);
}
