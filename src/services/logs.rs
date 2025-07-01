use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_logs(path: &str, max_lines: usize) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok) // instead of flatten()
        .collect();

    let last_lines = lines
        .iter()
        .rev()
        .take(max_lines)
        .rev()
        .cloned()
        .collect::<Vec<String>>();

    last_lines.join("\n")
}
