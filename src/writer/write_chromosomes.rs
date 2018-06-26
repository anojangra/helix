use forge::Chromosome;
use repo;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use writer;

/// Write chromosomes to disk
///
pub fn call(chromosomes: &Vec<Chromosome>) {
    debug!("writing chromsosome to disk");
    print!("#\n");
    io::stdout().flush().unwrap();
    let mut f = File::create("/tmp/chromosomes.txt").expect("Unable to create file");
    for chromosome in chromosomes {
        let c = chromosome;
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            c.id,
            c.target_ticker,
            c.chromosome,
            writer::fmt_vec_dna(c.dna.clone()),
            c.generation,
            c.chromosome_length,
            c.kelly,
            c.cum_pnl,
            c.variance,
            c.mean_return,
            c.w_kelly,
            c.num_of_trades,
            c.winning_trades,
            c.losing_trades,
            c.percentage_winners,
            c.rank
        ).unwrap();
    }
    repo::chromosomes::copy();
    fs::remove_file("/tmp/chromosomes.txt").unwrap();
}
