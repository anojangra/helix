pub mod write_signals;
pub mod write_chromosomes;

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

#[test]
fn test_fmt_vec_string() {
    let t = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!("\"{A,B,C}\"", fmt_vec_string(t))
}

#[test]
fn test_fmt_vec_dna() {
    let t: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    assert_eq!("\"{1,2,3,4,5,6,7,8,9,10,11,12}\"", fmt_vec_dna(t))
}
