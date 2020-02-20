#[macro_use]
extern crate log;
extern crate chrono;
extern crate csv;
extern crate env_logger;
extern crate forge;
extern crate serde;
extern crate uuid;

/// structs that map to db tables
pub mod schemas;
/// test sql statements
pub mod sql;

use csv::Reader;
use schemas::Quote;
use schemas::Return;

/// Get quotes for ticker symbol
pub fn get_quotes_by_symbol(ticker: &String, ticker_path: &str) -> Vec<Quote> {
  let mut quotes: Vec<Quote> = vec![];
  let filepath = format!("{}/data/{}.csv", ticker_path, ticker);
  let mut rdr = Reader::from_path(filepath).expect(&*format!("No file for {}", ticker));

  for row in rdr.records() {
    let record = row.unwrap();
    // TODO: There has to be a cleaner way to do this.
    let quote = Quote {
      ticker: record.get(0).unwrap().to_string(),
      ts: record.get(1).unwrap().parse().unwrap(),
      open: record.get(2).unwrap().parse().unwrap(),
      high: record.get(3).unwrap().parse().unwrap(),
      low: record.get(4).unwrap().parse().unwrap(),
      close: record.get(5).unwrap().parse().unwrap(),
      volume: record.get(6).unwrap().parse().unwrap(),
    };
    quotes.push(quote);
  }

  quotes
}

/// Get returns
/// Gets returns from the of the target ticker
/// Accepts the path to the csv as its parameter
pub fn get_returns(target_returns_path: &str) -> Vec<Return> {
  let mut target_returns: Vec<Return> = vec![];
  let mut rdr = Reader::from_path(target_returns_path)
    .expect("Couldn't open target returns file. Make sure you entered the full path\n");
  for row in rdr.records() {
    let record: std::result::Result<csv::StringRecord, csv::Error> = row;
    debug!("{:?}", record);
    let r = match record {
      Ok(v) => Return {
        ts: v.get(0).unwrap().parse().unwrap(),
        ret: v.get(1).unwrap().parse().unwrap(),
      },
      Err(msg) => panic!(msg),
    };
    target_returns.push(r);
  }
  target_returns
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_get_quotes_by_symbol() {
    let quotes = get_quotes_by_symbol(&"MMM".to_string(), "/home/choiway/data/spx_data_relix/data");
    let first_quote = &quotes[0];
    assert_eq!(first_quote.ticker, "MMM");
  }

  #[test]
  fn test_get_returns() {
    let returns = get_returns("/home/choiway/data/spx_data_relix/aapl_returns.csv");
    let first_ret = &returns[0];
    assert_eq!(first_ret.ts, 946944000.0);
  }
}
