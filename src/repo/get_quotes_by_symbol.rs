use repo::pg;
use repo::sql::get_quotes_by_symbol;
use schemas::Quote;

pub fn call(ticker: &String) -> Vec<Quote> {
    let conn = pg::connect();
    let rows = &conn.query(get_quotes_by_symbol::sql(), &[&ticker]).unwrap();
    let mut quotes: Vec<Quote> = vec![];

    for row in rows {
        let quote = Quote {
            ticker: row.get(0),
            ts: row.get(1),
            open: row.get(2),
            high: row.get(3),
            low: row.get(4),
            close: row.get(5),
            volume: row.get(6),
        };
        quotes.push(quote);
    }

    quotes
}
