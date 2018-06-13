use chromosome::Chromosome;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use trade_signal::TradeSignal;

/// Write signals to disk
///
pub fn call(signals: BTreeMap<String, TradeSignal>, chromosome: &Chromosome) {
    let mut f = File::create("/tmp/output.txt").expect("Unable to create file");
    for signal in signals {
        let s = signal.1;
        write!(
            f,
            "{},{},{}\n",
            s.chromosome_id,
            s.ts,
            fmt_vec_string(s.strategies)
        ).unwrap();
    }
}

/// Format vector of String
///
/// Formats the vector to be readable by postgresql as an array
///
fn fmt_vec_string(strings: Vec<String>) -> String {
    let mut strings = strings;
    let mut s = String::from("\"{");
    s.push_str(strings.remove(0).as_str());
    for string in strings {
        s.push_str(",");
        s.push_str(string.as_str());
    }
    let close_brace = "}\"";
    s.push_str(close_brace);
    s
}

#[test]
fn test_fmt_vec_string() {
    let t = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!("\"{A,B,C}\"", fmt_vec_string(t))
}
