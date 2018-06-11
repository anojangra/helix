#[derive(Debug)]
pub struct Ticker {
    pub symbol: String,
}

#[derive(Debug)]
pub struct Quote {
    pub ticker: String,
    pub ts: f64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}


