use forge::Chromosome;
use repo;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use trade_signal::TradeSignal;
use writer;

/// Write signals to disk
pub fn call(signals: &BTreeMap<String, TradeSignal>, chromosome: &Chromosome) {
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
            writer::fmt_vec_string(s.strategies.clone()),
            writer::fmt_vec_dna(s.signals.clone()),
            s.target_ticker,
            s.hard_signal,
            s.generation,
            s.ret,
            s.pnl
        ).unwrap();
    }
    repo::trade_signals::copy(&filename);
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
