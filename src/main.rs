//! # Helix is awesome
//!
//! Where does the rest of this go?
//!
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
use std::sync::mpsc::channel;
use std::thread;

pub fn main() {
    env_logger::init();
    info!("Hello, world!");
    // Init
    let quotes_repo = init_quotes_repo();
    let dnas = forge::generate_dnas(12, config::POPULATION_SIZE);
    let returns = init_returns();
    repo::init_trade_signals();
    repo::init_chromosomes();
    let mut ranked_chromosomes: Vec<Chromosome> = vec![];
    // Run generations
    for i in 1..4 {
        warn!("Running generation: {}", i);
        // Generate or evolve chromosomes
        let mut chromosomes: Vec<Chromosome> = vec![];
        if i == 1 {
            chromosomes = forge::generate_chromosomes(dnas.clone(), i, config::TARGET_TICKER)
        } else {
            chromosomes = forge::evolve(ranked_chromosomes, i);
        }
        let c_len = *&chromosomes.len();
        // Init channel to collect updated chromosomes
        let (c_tx, c_rx) = channel();
        // Init throttle
        // The throttle limits the number of chromosomes that are processed at 
        // any given time
        let (throttle_tx, throttle_rx) = crossbeam_channel::bounded(8);
        // Process chromosomes and collect results in channel
        for chromosome in chromosomes {
            let q_clone = quotes_repo.clone();
            let r_clone = returns.clone();
            let tx_n = c_tx.clone();
            let t_rx = throttle_rx.clone();
            // Send integer to throttle to start count
            throttle_tx.send(1);
            debug!("Throttle length: {}", throttle_rx.len());
            thread::spawn(move || {
                tx_n.send(process_chromosome(&chromosome, q_clone, r_clone))
                    .unwrap();
                t_rx.recv().unwrap();
            });
        }

        let updated_chromosomes: Vec<Chromosome> = c_rx.iter().take(c_len).map(|c| c).collect();
        ranked_chromosomes = rank_chromosomes(updated_chromosomes);
        writer::write_chromosomes(&ranked_chromosomes);
    }
    info!("So long and thanks for all the fish!");
}

/// Initializes hashmap for quotes
fn init_quotes_repo() -> HashMap<String, Vec<Quote>> {
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
