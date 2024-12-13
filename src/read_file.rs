use std::io::{BufRead, BufReader};

use anyhow::Error;

pub fn read_input_file(file_src: &str) -> Result<Vec<Vec<u64>>, Error> {
    let file = std::fs::OpenOptions::new()
        .write(false)
        .read(true)
        .open(file_src)?;

    let mut report_vec: Vec<Vec<u64>> = Vec::new();

    let file_reader = BufReader::new(file);
    for line in file_reader.lines() {
        let line_string = line.expect("unable to get line");
        let report: Vec<u64> = line_string
            .split_whitespace()
            .map(|s| s.trim().parse().expect("unable to parse int from string"))
            .collect();

        report_vec.push(report);
    }

    Ok(report_vec)
}
