//! # Forge where
//!
//! Where does the rest of this go?
//!
extern crate uuid;
extern crate rand;

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
