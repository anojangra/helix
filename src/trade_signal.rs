extern crate uuid;
use uuid::Uuid;
use strategies::Strategy;
use schemas::Quote;

#[derive(Debug)]
pub struct TradeSignal {
    pub chromosome_id: Uuid,
    pub ts: f64,
    pub strategies: Vec<String>,
    pub signals: Vec<i64>,
    pub target_ticker: String,
    pub hard_signal: i64,
    pub generation: i64,
}

pub fn init_trade_signal(strategy: Strategy, quote: &Quote, signal: i64) -> TradeSignal {
    let strategies = vec![strategy.strategy];
    let signals = vec![signal];
    TradeSignal {
        chromosome_id: strategy.chromosome_id,
        ts: quote.ts,
        strategies: strategies,
        signals: signals,
        target_ticker: strategy.target_ticker,
        hard_signal: 0,
        generation: strategy.generation,
    }
}