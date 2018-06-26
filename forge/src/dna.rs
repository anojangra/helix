//! dna type
//!
//! Where does the rest of this go?
//!

use rand::prelude::*;

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

// Generates random dna
fn generate_dna(len: i32) -> Dna {
    let mut dna: Vec<i32> = vec![];
    let mut rng = thread_rng();
    for _i in 0..len {
        let base: i32 = rng.gen_range(1, 256);
        dna.push(base);
    }
    dna
}

#[test]
fn test_generate() {
    let dna = generate(12);
    assert_eq!(12, dna.len());
}

#[test]
fn test_generate_dnas() {
    let dnas = generate_dnas(12, 10000);
    assert_eq!(10000, dnas.len());
    assert_eq!(12, dnas[0].len())
}
