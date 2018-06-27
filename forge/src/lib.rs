//! # Forge where
//!
//! Where does the rest of this go?
//!
extern crate rand;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;

use rand::prelude::*;
use uuid::Uuid;


mod config;

/// chromosome type
#[derive(Debug, Clone)]
pub struct Chromosome {
    pub id: Uuid,
    pub target_ticker: String,
    pub chromosome: String,
    pub dna: Vec<i32>,
    pub generation: i32,
    pub chromosome_length: i32,
    pub kelly: f32,
    pub cum_pnl: f32,
    pub variance: f32,
    pub mean_return: f32,
    pub w_kelly: f32,
    pub num_of_trades: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub percentage_winners: f32,
    pub rank: i32,
}

/// Generate chromosomes
///
/// ## Example
/// ```
///
/// ```
pub fn generate_chromosomes(dnas: Vec<Dna>, generation: i32, ticker: &str) -> Vec<Chromosome> {
    // debug!("generate chromosomes");
    let mut chromosomes: Vec<Chromosome> = vec![];
    for dna in dnas {
        let strategies = decode_dna("<code>".to_string(), &dna);
        let strategies_vec: &Vec<&str> = &strategies.split("::").collect();
        let chromosome = Chromosome {
            id: uuid::Uuid::new_v4(),
            target_ticker: ticker.to_string(),
            chromosome: strategies.clone(),
            dna: dna,
            generation: generation,
            chromosome_length: strategies_vec.len() as i32,
            kelly: 0.0,
            cum_pnl: 0.0,
            variance: 0.0,
            mean_return: 0.0,
            w_kelly: 0.0,
            num_of_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            percentage_winners: 0.0,
            rank: 0,
        };
        chromosomes.push(chromosome);
    }
    chromosomes
}

pub fn decode_dna(code: String, dna: &Dna) -> String {
    let mut code = code;
    for base in dna {
        code = expand_code(code, base);
    }
    code = code.replace("::<code>", "");
    code
}

/// Expands dna to code
///
fn expand_code(code: String, base: &i32) -> String {
    if code.contains("<ticker>") {
        let index = base % config::tickers_length();
        return code.replace("<ticker>", config::TICKERS[index as usize]);
    };
    if code.contains("<param>") {
        return code.replace("<param>", &base.to_string());
    };
    if code.contains("<stmnt>") {
        let index = base % config::strategies_length();
        return code.replace("<stmnt>", config::STRATEGIES[index as usize]);
    };
    if code.contains("<code>") {
        let index = base % config::statements_length();
        return code.replace("<code>", config::STATEMENTS[index as usize]);
    }
    return code;
}

/// Dna type
/// The Dna type is alias for a vector of i32
pub type Dna = Vec<i32>;

/// Generates qty dnas of length len
pub fn generate_dnas(len: i32, qty: i32) -> Vec<Dna> {
    let mut dnas: Vec<Dna> = vec![];
    for _i in 0..qty {
        let dna = generate_dna(len);
        dnas.push(dna);
    }
    dnas
}

/// Generates random dna
fn generate_dna(len: i32) -> Dna {
    let mut dna: Vec<i32> = vec![];
    let mut rng = thread_rng();
    for _i in 0..len {
        let base: i32 = rng.gen_range(1, 256);
        dna.push(base);
    }
    dna
}

pub fn evolve(ranked_chromosomes: Vec<Chromosome>, generation: i32) -> Vec<Chromosome> {
    let start = &ranked_chromosomes.len() - config::FITTEST;
    let fittest_chromosomes = &ranked_chromosomes[start..];
    let pool = generate_pool(fittest_chromosomes);
    let dnas = mate(&pool);
    generate_chromosomes(dnas.clone(), generation, config::TARGET_TICKER)
}

fn generate_pool(ranked_chromosomes: &[Chromosome]) -> Vec<Dna> {
    debug!{"generate pool"};
    let mut pool: Vec<Dna> = Vec::new();
    for c in ranked_chromosomes {
        for _i in 0..c.rank {
            pool.push(c.dna.clone());
        }
    }
    pool
}

fn mate(pool: &Vec<Dna>) -> Vec<Dna> {
    debug!("mate");
    let mut rng = thread_rng();
    let mut new_dnas: Vec<Dna> = Vec::new();
    for _i in 0..config::POPULATION_SIZE {
        let x_dna = get_random_dna(&pool);
        let y_dna = get_random_dna(&pool);
        let splice_point = rng.gen_range(0, x_dna.len());
        let offspring = mutate_dna(crossover(x_dna, y_dna, splice_point));
        new_dnas.push(offspring);
    }
    new_dnas
}

fn get_random_dna(pool: &Vec<Dna>) -> Dna {
    let mut rng = thread_rng();
    let idx = rng.gen_range(0, pool.len());
    pool[idx].clone()
}

fn crossover(x: Dna, y: Dna, splice_point: usize) -> Dna {
    let papa = &x[..splice_point];
    let mama = &y[splice_point..];
    let mut offspring: Dna = vec![];
    offspring.extend(papa);
    offspring.extend(mama);
    debug!("offspring len: {}", offspring.len());
    offspring
}

// Mutates each base in dna based on MUTATE PROB
fn mutate_dna(dna: Dna) -> Dna {
    let mut mutated_dna: Dna = vec![];
    for base in dna {
        let mut b: i32;
        let p = rand::random::<f32>();
        if p < config::MUTATE_PROB {
            b = mutate_base(base);
        } else {
            b = base;
        }
        mutated_dna.push(b);
    }
    mutated_dna
}

// Either increments or decrements base by 1
fn mutate_base(base: i32) -> i32 {
    if rand::random() {
        return floor(base + 1);
    }
    return floor(base - 1);
}

// Ensures that we don't get a number less than 1
// A parameter of zero might end up with current day calculations
fn floor(base: i32) -> i32 {
    if base < 1 {
        return 1;
    }
    return base;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let dna = generate_dna(12);
        assert_eq!(12, dna.len());
    }

    #[test]
    fn test_generate_dnas() {
        let dnas = generate_dnas(12, 10000);
        assert_eq!(10000, dnas.len());
        assert_eq!(12, dnas[0].len())
    }

    #[test]
    fn test_modulo() {
        let x = 11 % 4;
        assert_eq!(3, x);
        let y = 12 % 4;
        assert_eq!(0, y)
    }

    #[test]
    fn test_decode_dna() {
        let dna = vec![241, 252, 253, 8, 13, 118, 184, 1, 225, 54, 141, 95];
        let chromosome = decode_dna("<code>".to_string(), &dna);
        assert_eq!(
            "hhv:btceUSD:8::stdevd:bitstampUSD:1::abovema:krakenEUR:95",
            chromosome
        );
    }
}
