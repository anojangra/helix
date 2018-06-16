use chromosome::Chromosome;
use config;
use dna::Dna;
use rand;
use rand::prelude::*;
use controls;

pub fn evolve(ranked_chromosomes: Vec<Chromosome>, generation: i32) -> Vec<Chromosome> {
    let start = &ranked_chromosomes.len() - config::FITTEST;
    let fittest_chromosomes = &ranked_chromosomes[start..];
    let pool = generate_pool(fittest_chromosomes);
    let dnas = mate(&pool);
    controls::generate_chromosomes(dnas.clone(), generation, config::TARGET_TICKER)
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
        return zero_floor(base + 1)       
    }
    return zero_floor(base - 1);
}

// Ensures that we don't get a negative number
fn zero_floor(base: i32) -> i32 {
    if base < 0 {
        return 0;
    }
    return base;
}