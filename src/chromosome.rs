extern crate uuid;
use config;
use dna;
use uuid::Uuid;

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
    pub rank: i32,
}

pub fn decode_dna(code: String, dna: &dna::Dna) -> String {
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
        "hhv:btceUSD:8::llv:bitstampUSD:1::hhv:krakenEUR:95",
        chromosome
    );
}
