use chrono;
use chrono::prelude::*;
use uuid;

#[derive(Debug)]
pub struct Ticker {
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct Quote {
    pub ticker: String,
    pub ts: f64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}

#[derive(Debug, Clone)]
pub struct TradeRank {
    pub id: uuid::Uuid,
    pub chromosome: String,
    pub dna: Vec<i32>,
    pub kelly: f32,
    pub cum_pnl: f32,
    pub variance: f32,
    pub mean_return: f32,
    pub w_kelly: f32,
    pub num_of_trades: f32,
    pub rank: f32,
    pub generation: f32,
}

#[derive(Debug, Clone)]
pub struct TradePnl {
    pub chromosome_id: uuid::Uuid,
    pub ts: chrono::DateTime<Utc>,
    pub strategies: Vec<String>,
    pub ticker: String,
    pub signal: i32,
    pub pnl: f32,
    pub cum_pnl: f32,
    pub generation: i32,
}
