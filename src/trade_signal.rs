extern crate uuid;
use strategies::Strategy;
use strategies::Window;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TradeSignal {
    pub chromosome_id: Uuid,
    pub ts: f64,
    pub strategies: Vec<String>,
    pub signals: Vec<i32>,
    pub target_ticker: String,
    pub hard_signal: i32,
    pub generation: i32,
    pub pnl: f32,
}

/// Initializes empty trade signal
///
pub fn init_trade_signal(strategy: Strategy, window: &Window, signal: i32) -> TradeSignal {
    let strategies = vec![strategy.strategy];
    let signals = vec![signal];
    TradeSignal {
        chromosome_id: strategy.chromosome_id,
        ts: window.current_quote.ts,
        strategies: strategies,
        signals: signals,
        target_ticker: strategy.target_ticker,
        hard_signal: 0,
        generation: strategy.generation,
        pnl: 0.0,
    }
}
