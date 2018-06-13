use chromosome::Chromosome;
use std::fs::File;
use std::io::Write;
use writer;

/// Write chromosomes to disk
///
pub fn call(chromosomes: &Vec<Chromosome>) {
    let mut f = File::create("/tmp/chromosomes.txt").expect("Unable to create file");
    for chromosome in chromosomes {
        let c = chromosome;
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            c.id,
            c.target_ticker,
            c.chromosome,
            writer::fmt_vec_dna(c.dna.clone()),
            c.generation,
            c.chromosome_length,
        ).unwrap();
    }
}