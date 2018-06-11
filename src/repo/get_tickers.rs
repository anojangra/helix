use repo::pg;
use repo::sql::get_tickers;
use schemas::Ticker;

pub fn call() -> Vec<Ticker> {
    let conn = pg::connect();
    let rows = &conn.query(get_tickers::sql(), &[]).unwrap();
    let mut tickers: Vec<Ticker> = vec![];
    for row in rows {
        let ticker = Ticker { symbol: row.get(0) };
        tickers.push(ticker);
    }
    tickers
}
