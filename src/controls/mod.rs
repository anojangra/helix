pub mod write_signals;

use chromosome;
use chromosome::Chromosome;
use dna::Dna;
use uuid;


/// Generate chromosomes from dnas
///
pub fn generate_chromosomes(dnas: Vec<Dna>, generation: i32, ticker: &str) -> Vec<Chromosome> {
    let mut chromosomes: Vec<Chromosome> = vec![];
    for dna in dnas {
        let strategies = chromosome::decode_dna("<code>".to_string(), &dna);
        let strategies_vec: &Vec<&str> = &strategies.split("::").collect();
        let chromosome = Chromosome {
            id: uuid::Uuid::new_v4(),
            target_ticker: ticker.to_string(),
            chromosome: strategies.clone(),
            dna: dna,
            generation: generation,
            chromosome_length: strategies_vec.len() as i32,
        };
        chromosomes.push(chromosome);
    }
    chromosomes
}
