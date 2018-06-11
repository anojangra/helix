use schemas::Quote;
use std::collections::BTreeMap;
use strategies::Strategy;
use trade_signal::TradeSignal;
use strategies::insert_signal;


pub fn call(
    strategy: Strategy,
    trade_signals: BTreeMap<String, TradeSignal>,
    quotes: &Vec<Quote>,
) -> BTreeMap<String, TradeSignal> {
    let mut updated_trade_signals = trade_signals;
    for quote in quotes {
        let signal = 1;
        updated_trade_signals = insert_signal(updated_trade_signals, quote, strategy.clone(), signal);
    }
    updated_trade_signals
}
