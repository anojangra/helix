extern crate postgres;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate forge;
extern crate uuid;

pub mod schemas;
pub mod sql;

use forge::Chromosome;
use postgres::{Connection, TlsMode};
use schemas::Ticker;
use schemas::Return;
use schemas::Quote;

pub fn connect() -> Connection {
    Connection::connect(
        "postgres://hugo:InRainbows@localhost:5432/hugo",
        TlsMode::None,
    ).unwrap()
}

/// Initialize trade signals table
pub fn init_trade_signals() {
    info!("Init trade signals table");
    let conn = connect();
    conn.batch_execute(
        "   DROP TABLE IF EXISTS trade_signals;
            CREATE TABLE trade_signals (
                chromosome_id uuid
            ,   ts integer not null
            ,   strategies text array
            ,   signals integer array
            ,   target_ticker text
            ,   hard_signal integer
            ,   generation integer
            ,   ret numeric
            ,   pnl numeric
            );
            CREATE INDEX ON trade_signals (chromosome_id, ts);",
    ).unwrap();
}

/// Initialize chromosomes table
pub fn init_chromosomes() {
    info!("Init chromosomes table");
    let conn = connect();
    conn.batch_execute(
        "   DROP TABLE IF EXISTS trade_chromosomes;
            CREATE TABLE trade_chromosomes (
            id uuid,
            target_ticker text, 
            chromosome text,
            dna integer array,
            generation int,
            chromosome_length int,
            kelly numeric,
            cum_pnl numeric,
            variance numeric,
            mean_return numeric,
            w_kelly numeric,
            num_of_trades integer,
            winning_trades integer,
            losing_trades integer,
            percentage_winners numeric,
            rank integer
            );",
    ).unwrap();
}

/// Copy chromosomes
pub fn copy_chromosomes() {
    let conn = connect();
    conn.execute("COPY trade_chromosomes FROM '/tmp/chromosomes.txt';", &[])
        .unwrap();
}

/// Insert chromosome
pub fn insert_chromosome(c: Chromosome) {
    let conn = connect();
    conn.execute(
        sql::insert_chromosome(),
        &[
            &c.id,
            &c.target_ticker,
            &c.chromosome,
            &c.dna,
            &c.generation,
            &c.chromosome_length,
        ],
    ).unwrap();
}

pub fn copy_signals(filename: &String) {
    let conn = connect();
    let sql = format!("COPY trade_signals FROM '{}';", filename);
    conn.execute(&sql, &[]).unwrap();
}

pub fn get_quotes_by_symbol(ticker: &String) -> Vec<Quote> {
    let conn = connect();
    let rows = &conn.query(sql::get_quotes_by_symbol(), &[&ticker]).unwrap();
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

/// Get returns
///
pub fn get_returns(ticker: String) -> Vec<Return> {
    let conn = connect();
    let rows = &conn.query(sql::get_returns(), &[&ticker]).unwrap();
    let mut target_returns: Vec<Return> = vec![];
    for row in rows {
        let ret: Option<Result<f32, postgres::Error>> = row.get_opt(1);
        let r = match ret {
            None => panic!("No value"),
            Some(Ok(_v)) => Return {
                ts: row.get("ts"),
                ret: row.get("ret"),
            },
            Some(Err(_msg)) => Return {
                ts: row.get("ts"),
                ret: 0.0,
            },
        };
        target_returns.push(r);
    }
    target_returns
}

/// Get tickers
pub fn get_tickers() -> Vec<Ticker> {
    let conn = connect();
    let rows = &conn.query(sql::get_tickers(), &[]).unwrap();
    let mut tickers: Vec<Ticker> = vec![];
    for row in rows {
        let ticker = Ticker { symbol: row.get(0) };
        tickers.push(ticker);
    }
    tickers
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
