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
#[macro_use]
extern crate log;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate forge;
extern crate repo;
extern crate vger;
extern crate writer;

mod config;

use forge::Chromosome;
use repo::schemas::Quote;
use repo::schemas::Return;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn main() {
    env_logger::init();
    info!("Hello, world!");

    // Init sequence
    let quotes_repo = init_quotes_repo();
    let mut completed_chromosomes = init_completed_chromosomes();
    let returns = init_returns();
    repo::init_trade_signals();
    repo::init_chromosomes();
    let mut ranked_chromosomes: Vec<Chromosome> = vec![];

    // Run generations
    for generation in 1..4 {
        let mut chromosomes = generate_chromosomes(ranked_chromosomes, generation);
        let chromosomes_len = *&chromosomes.len();
        let (chromosomes_tx, chromosomes_rx) = init_chromosomes_channel();
        let (throttle_tx, throttle_rx) = init_throttle(8);
        // Check completed chromosomes
        process_chromosomes(
            chromosomes,
            &mut completed_chromosomes,
            &quotes_repo,
            &returns,
            chromosomes_tx,
            throttle_tx,
            throttle_rx,
        );

        let updated_chromosomes: Vec<Chromosome> = chromosomes_rx
            .iter()
            .take(chromosomes_len)
            .map(|c| c)
            .collect();

        ranked_chromosomes = rank_chromosomes(updated_chromosomes);
        writer::write_chromosomes(&ranked_chromosomes);
    }

    info!("So long and thanks for all the fish!");
}

/// Initializes hashmap for quotes
fn init_quotes_repo() -> HashMap<String, Vec<Quote>> {
    debug!("Initializing quotes repo");

    let mut repo = HashMap::new();

    for ticker in repo::get_tickers() {
        debug!("{:?}", ticker);
        let quotes = repo::get_quotes_by_symbol(&ticker.symbol);
        repo.insert(ticker.symbol, quotes);
    }

    repo
}

/// Initializes Btreemap for returns
fn init_returns() -> BTreeMap<String, Return> {
    debug!("Initializing returns");

    let mut repo: BTreeMap<String, Return> = BTreeMap::new();

    for ret in repo::get_returns(config::TARGET_TICKER.to_string()) {
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
fn generate_chromosomes(ranked_chromosomes: Vec<Chromosome>, generation: i32) -> Vec<Chromosome> {
    warn!("Running generation: {}", generation);

    if generation == 1 {
        let dnas = forge::generate_dnas(12, config::POPULATION_SIZE);
        return forge::generate_chromosomes(dnas.clone(), generation, config::TARGET_TICKER);
    } else {
        return forge::evolve(ranked_chromosomes, generation);
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
) {
    for chromosome in chromosomes {
        let q_clone = quotes_repo.clone();
        let r_clone = returns.clone();
        let chromosome_chan = chromosome_tx.clone();
        let throttle = throttle_rx.clone();

        throttle_tx.send(1); // The value doesn't matter

        debug!("Throttle length: {}", throttle_rx.len());

        if completed_chromosomes.contains_key(&chromosome.chromosome) == false {
            completed_chromosomes.insert(chromosome.chromosome.clone(), chromosome.clone());
            thread::spawn(move || {
                chromosome_chan
                    .send(process_chromosome(&chromosome, q_clone, r_clone))
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
) -> Chromosome {
    let mut trade_signals = vger::generate_signals(&chromosome, quotes_repo);
    vger::merge_returns(&mut trade_signals, &returns);
    vger::calc_pnl(&mut trade_signals, chromosome.clone());
    writer::write_signals(&trade_signals, &chromosome);
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
