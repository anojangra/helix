//! An implementation of a genetic algortihm that use grammtical evolution
//! to find trading signals
//!
//! # Overview
//!
//! Helix takes data from a table with the following structure
//!
//! ```
//!  Column |            Type             | Collation | Nullable | Default
//! --------+-----------------------------+-----------+----------+---------
//!  ticker | text                        |           |          |
//!  ts     | timestamp without time zone |           | not null |
//!  open   | numeric                     |           |          |
//!  high   | numeric                     |           |          |
//!  low    | numeric                     |           |          |
//!  close  | numeric                     |           |          |
//!  volume | numeric                     |           |          |
//!
//! ```
//!
//! This would be a "tall" table of securities price data.
//!
//! Here's a summary of local crates:
//! * Forge - genetic algorthims
//! * Vger - trade algorithms
//! * Writer - writes to disk
//! * Repo - save to database
//!
//!
//! ## Config
//!
//! ### Helix Config
//!
//! Set up requires you to set config `TARGET_TICKER`, `FITTEST` and `POPULATION_SIZE`
//! * `TARGET_TICKER` - the ticker symbol of the security your looking for trading signals for
//! * `FITTEST` - the number of fittest chromosomes to use for the pool in the next generation
//! * `POPULATION_SIZE` - the population size of the pool
//!
//! ### Forge Config
//!
//! You'll need to update the the tickers in the forge config file with the universe of symbols
//! that you'll use in genetic algorithms.
//! ```
//! pub static TICKERS: [&str; 13] = [
//!	"coinbaseUSD",
//!	"zaifJPY",
//!	"bitstampUSD",
//!	"coincheckJPY",
//!	"btcnCNY",
//!	"bitflyerJPY",
//!	"btceUSD",
//!	"btctradeCNY",
//!	"coinbaseEUR",
//!	"bitfinexUSD",
//!	"fiscoJPY",
//!	"krakenEUR",
//!	"krakenUSD"];
//! ```
extern crate clap;
#[macro_use]
extern crate log;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate forge;
extern crate glob;
extern crate repo;
extern crate vger;
extern crate writer;

mod config;

use clap::{App, Arg};
use forge::Chromosome;
use glob::{glob_with, MatchOptions};
use repo::schemas::Quote;
use repo::schemas::Return;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn main() {
  let matches = App::new("helix")
    .version("v0.1-beta")
    .author("choiway <waynechoi@gmail.com>")
    .about("Genetic Algorithm for Financial Data")
    .arg(
      Arg::with_name("threads")
        .short("t")
        .long("threads")
        .value_name("THREADS")
        .help("Sets the number of threads to use")
        .required(true),
    )
    .arg(
      Arg::with_name("target_ticker")
        .short("s")
        .long("target_ticker")
        .value_name("TARGET_TICKER")
        .help("The ticker of the security you are trying to predict (i.e. SPY, AAPL, coinbaseUSD)")
        .required(true),
    )
    .arg(
      Arg::with_name("pool_description")
        .short("d")
        .long("pool_description")
        .value_name("DESCRIPTION")
        .help("Description of the pool of securities (i.e. SP500, btc-exchanges)")
        .required(true),
    )
    .arg(
      Arg::with_name("repo_pathname")
        .short("p")
        .long("repo_pathname")
        .value_name("PATH")
        .help("Path to work directory. Should have a *data* directory as a sub directory")
        .required(true),
    )
    .arg(
      Arg::with_name("returns_filename")
        .short("r")
        .value_name("FILENAME")
        .help("Filename of the target returns to predict. Should be located in the repo")
        .required(true),
    )
    .get_matches();

  // Parse arguments

  let num_of_threads: usize = matches.value_of("threads").unwrap_or("4").parse().unwrap();
  info!("Number of threads: {}", num_of_threads);
  let target_ticker: &str = matches.value_of("target_ticker").unwrap();
  info!("Target Ticker: {}", target_ticker);
  let pool_description: &str = matches.value_of("pool_description").unwrap();
  info!("Pool Description: {}", pool_description);
  let backtest_id: String = generate_backtest_id(pool_description, target_ticker);
  info!("Backtest ID: {}", backtest_id);
  let repo_path: &str = matches.value_of("repo_pathname").unwrap();
  info!("Repo Path: {}", repo_path);
  let returns_filename = matches.value_of("returns_filename").unwrap();
  let target_returns_path: &str = &format!("{}{}", repo_path, returns_filename);
  debug!("Target returns path: {}", target_returns_path);

  // Init sequence
  
  env_logger::init();
  info!("Starting grammatical revolution");
  info!("Initializing tickers");
  let tickers = get_tickers(repo_path);
  info!("Initializing quotes repo");
  let quotes_repo = init_quotes_repo(&tickers, repo_path);
  info!("Initializing chromosomes");
  let mut completed_chromosomes = init_completed_chromosomes();
  info!("Initializing returns");
  let returns = init_returns(target_returns_path);
  info!("Initializing ranked chromosomes");
  let mut ranked_chromosomes: Vec<Chromosome> = vec![];

  // Run generations
  //
  // [WHC] If you ever decide to figure out how to run this across a cluster
  // you'll have to extract this to a separate machine and figure out
  // how to coordinate all the threads on different nodes.
  // Good luck!
  for generation in 1..4 {
    let chromosomes = generate_chromosomes(ranked_chromosomes, generation, &tickers, target_ticker);
    let chromosomes_len = *&chromosomes.len();
    let (chromosomes_tx, chromosomes_rx) = init_chromosomes_channel();
    let (throttle_tx, throttle_rx) = init_throttle(num_of_threads);
    // Check completed chromosomes
    process_chromosomes(
      chromosomes,
      &mut completed_chromosomes,
      &quotes_repo,
      &returns,
      chromosomes_tx,
      throttle_tx,
      throttle_rx,
      &backtest_id,
    );

    let updated_chromosomes: Vec<Chromosome> = chromosomes_rx
      .iter()
      .take(chromosomes_len)
      .map(|c| c)
      .collect();

    ranked_chromosomes = rank_chromosomes(updated_chromosomes);
    writer::write_chromosomes(&ranked_chromosomes, generation, &backtest_id);
  }

  info!("So long and thanks for all the fish!");
}

// Creates a vector of tickers from CSV file names
// filenames should be named in the following format:
// AAPL.csv
fn get_tickers(repo_path: &str) -> Vec<String> {
  let data_path: &str = &format! {"{}data/*.csv", repo_path};
  debug!("Opening tickers at path: {}", data_path);
  let options = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
  };
  let mut tickers: Vec<String> = Vec::new();
  for entry in glob_with(data_path, options).unwrap() {
    if let Ok(path) = entry {
      let filename: &str = path.file_stem().unwrap().to_str().unwrap();
      tickers.push(filename.to_string());
    }
  }
  tickers
}

// The backtest id should take the following form
// TARGET_TICKER-TICKER_POOL
// i.e. SPX-SP500
// We add epoch to differentiate between different runs of the same target-pool
//
fn generate_backtest_id(pool_description: &str, target_ticker: &str) -> String {
  let id = format!("{}::{}", pool_description, target_ticker);
  let start = SystemTime::now();
  let epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  format!("{}-{:?}", id, epoch)
}

/// Initializes hashmap for quotes
///
/// The quotes repo
fn init_quotes_repo(tickers: &Vec<String>, repo_path: &str) -> HashMap<String, Vec<Quote>> {
  debug!("Initializing quotes repo");

  let mut repo = HashMap::new();

  for ticker in tickers {
    debug!("{:?}", ticker);
    let quotes = repo::get_quotes_by_symbol(&ticker, repo_path);
    repo.insert(ticker.clone(), quotes);
  }

  repo
}

/// Initializes Btreemap for returns
fn init_returns(target_returns_path: &str) -> BTreeMap<String, Return> {
  debug!("Initializing returns");

  let mut repo: BTreeMap<String, Return> = BTreeMap::new();

  for ret in repo::get_returns(target_returns_path) {
    let ts = ret.ts.to_string();
    repo.insert(ts, ret);
  }

  repo
}

/// Initalizes hashmap for complete chromosomes
///
/// In order to eliminate duplicated chromosomes, we create a hashmap to keep track of completed strategies
/// with `key` strategy and `value` chromosome. This helps in later generations.
fn init_completed_chromosomes() -> HashMap<String, Chromosome> {
  debug!("Initialize chromosomes map");
  HashMap::new()
}

/// Generate or evolve chromosomes
fn generate_chromosomes(
  ranked_chromosomes: Vec<Chromosome>,
  generation: i32,
  tickers: &Vec<String>,
  target_ticker: &str,
) -> Vec<Chromosome> {
  warn!("Running generation: {}", generation);

  if generation == 1 {
    let dnas = forge::generate_dnas(12, config::POPULATION_SIZE);
    return forge::generate_chromosomes(dnas.clone(), generation, target_ticker, tickers);
  } else {
    return forge::evolve(ranked_chromosomes, generation, tickers, target_ticker);
  }
}

/// Init channel to collect updated chromosomes
fn init_chromosomes_channel() -> (Sender<Chromosome>, Receiver<Chromosome>) {
  channel()
}

/// Init throttle channel
///
/// The throttle limits the number of chromosomes that are processed at
/// any given time. A key here is that the channel is bounded which
/// blocks when the channel is full.
fn init_throttle(
  workers: usize,
) -> (
  crossbeam_channel::Sender<i32>,
  crossbeam_channel::Receiver<i32>,
) {
  crossbeam_channel::bounded(workers)
}

/// Process chromosomes
///
/// Nothing is returned. Instead chromosomes are collected after they are sent back to
/// the chromosome receiver channel outside of the `for` loop.
pub fn process_chromosomes(
  chromosomes: Vec<Chromosome>,
  completed_chromosomes: &mut HashMap<String, Chromosome>,
  quotes_repo: &HashMap<String, Vec<Quote>>,
  returns: &BTreeMap<String, Return>,
  chromosome_tx: Sender<Chromosome>,
  throttle_tx: crossbeam_channel::Sender<i32>,
  throttle_rx: crossbeam_channel::Receiver<i32>,
  backtest_id: &String,
) {
  for chromosome in chromosomes {
    let q_clone = quotes_repo.clone();
    let r_clone = returns.clone();
    let chromosome_chan = chromosome_tx.clone();
    let throttle = throttle_rx.clone();
    let backtest_id_clone = backtest_id.clone();

    throttle_tx.send(1); // The value doesn't matter

    debug!("Throttle length: {}", throttle_rx.len());

    if completed_chromosomes.contains_key(&chromosome.chromosome) == false {
      print!(".");
      completed_chromosomes.insert(chromosome.chromosome.clone(), chromosome.clone());
      thread::spawn(move || {
        chromosome_chan
          .send(process_chromosome(
            &chromosome,
            q_clone,
            r_clone,
            backtest_id_clone,
          ))
          .unwrap();
        throttle.recv().unwrap();
      });
    } else {
      print!("*");
      io::stdout().flush().unwrap();
      throttle.recv().unwrap();
    }
  }
}

/// Generate signals and metadata for chromosome
pub fn process_chromosome(
  chromosome: &Chromosome,
  quotes_repo: HashMap<String, Vec<Quote>>,
  returns: BTreeMap<String, Return>,
  backtest_id: String,
) -> Chromosome {
  let mut trade_signals = vger::generate_signals(&chromosome, quotes_repo);
  vger::merge_returns(&mut trade_signals, &returns);
  vger::calc_pnl(&mut trade_signals, chromosome.clone());
  writer::write_signals(&trade_signals, &chromosome, backtest_id);
  vger::update_chromosome(chromosome.clone(), trade_signals)
}

/// Rank chromosomes by w_kelly
///
/// Rank is determined by the offset of the chromosomes.
///
/// ## Rank calculation
/// ```
/// Assume:
///
/// let x = Vec![1...20];
///
/// if fittest = 5 then the start idx = 5 and negative rank = 20 - 5 - 5 - 1 which
/// makes the starting index 9.
/// ```
pub fn rank_chromosomes(updated_chromosomes: Vec<Chromosome>) -> Vec<Chromosome> {
  let mut filtered_chromosomes: Vec<Chromosome> = updated_chromosomes
    .into_iter()
    .filter(|c| c.num_of_trades > 100)
    .collect();
  filtered_chromosomes.sort_by_key(|c| c.percentage_winners as i32);

  let end_idx = filtered_chromosomes.len() as i32;
  let fittest = config::FITTEST as i32;
  let start_idx = end_idx - fittest;

  for i in start_idx..end_idx {
    let chromosome = &mut filtered_chromosomes[i as usize];
    let negative_rank = (end_idx - i - fittest - 1) as i32;
    chromosome.rank = negative_rank.abs() as i32;
  }

  filtered_chromosomes.clone()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_open_tickers() {
    let tickers = open_tickers("test_tickers.txt");
    let first = &tickers[0];
    assert_eq!("MMM", first);
  }
}
