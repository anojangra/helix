extern crate env_logger;
extern crate forge;
extern crate vger;
#[macro_use]
extern crate log;
extern crate repo;


use std::collections::BTreeMap;
use forge::Chromosome;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use vger::TradeSignal;

/// Write chromosomes to disk
///
pub fn write_chromosomes(chromosomes: &Vec<Chromosome>) {
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
            fmt_vec_dna(c.dna.clone()),
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
    repo::copy_chromosomes();
    fs::remove_file("/tmp/chromosomes.txt").unwrap();
}

/// Format vector of String
///
/// Formats the vector to be readable by postgresql as an array
///
fn fmt_vec_string(strings: Vec<String>) -> String {
    let mut strings = strings;
    let mut s = String::from("{");
    s.push_str(strings.remove(0).as_str());
    for string in strings {
        s.push_str(",");
        s.push_str(string.as_str());
    }
    let close_brace = "}";
    s.push_str(close_brace);
    s
}

/// Format vector of i32
///
/// Formats the vector to be readable by postgresql as an array
///
fn fmt_vec_dna(dna: Vec<i32>) -> String {
    let mut dna = dna;
    let mut s = String::from("{");
    s.push_str(&dna.remove(0).to_string());
    for d in dna {
        s.push_str(",");
        s.push_str(&d.to_string());
    }
    let close_brace = "}";
    s.push_str(close_brace);
    s
}

/// Write signals to disk
pub fn write_signals(signals: &BTreeMap<String, TradeSignal>, chromosome: &Chromosome) {
    log_call(chromosome);
    let (filename, mut f) = create_file(chromosome);
    for signal in signals {
        let s = signal.1;
        debug!("writing signal: {:?} to disk", &s);
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            s.chromosome_id,
            s.ts,
            fmt_vec_string(s.strategies.clone()),
            fmt_vec_dna(s.signals.clone()),
            s.target_ticker,
            s.hard_signal,
            s.generation,
            s.ret,
            s.pnl
        ).unwrap();
    }
    repo::copy_signals(&filename);
    fs::remove_file(filename).unwrap();
}

fn log_call(chromosome: &Chromosome) {
    debug!("writing id: {} to disk", chromosome.id);
    print!(".");
    io::stdout().flush().unwrap();
}

fn create_file(chromosome: &Chromosome) -> (String, File) {
    let filename = format!("/tmp/{}.txt", chromosome.id);
    let file = File::create(&filename).expect("Unable to create file");
    return (filename, file);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
