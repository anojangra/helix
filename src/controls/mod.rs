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
            kelly: 0.0,
            cum_pnl: 0.0,
            variance: 0.0,
            mean_return: 0.0,
            w_kelly: 0.0,
            num_of_trades: 0,
            rank: 0,
        };
        chromosomes.push(chromosome);
    }
    chromosomes
}
