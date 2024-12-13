use std::io::{BufRead, BufReader};

use anyhow::{Error, Ok};

use crate::read_file::read_input_file;

fn read_file(file_src: &str) -> Result<(Vec<u64>, Vec<u64>), Error> {
    let file = std::fs::OpenOptions::new()
        .write(false)
        .read(true)
        .open(file_src)?;

    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    let file_reader = BufReader::new(file);
    for line in file_reader.lines() {
        let line_string = line.expect("unable to get line");
        let values: Vec<&str> = line_string.split_whitespace().collect();
        list1.push(
            values[0]
                .trim()
                .parse()
                .expect("unable to parse input integer from string"),
        );
        list2.push(
            values[1]
                .trim()
                .parse()
                .expect("unable to parse input integer from string"),
        );
    }
    list1.sort();
    list2.sort();

    return Ok((list1, list2));
}

fn create_input_lists() -> (Vec<u64>, Vec<u64>) {
    let input_file = "inputs/p1_input.txt";
    let line_vec = read_input_file(input_file).expect("unable to read input file");
    let vec1: Vec<u64> = line_vec.iter().map(|line| line[0]).collect();
    let vec2: Vec<u64> = line_vec.iter().map(|line| line[1]).collect();
    return (vec1, vec2);
}

pub fn p1() {
    let (list1, list2) = create_input_lists();

    assert_eq!(list1.len(), list2.len());

    let total_sum: u64 = list1
        .iter()
        .zip(list2)
        .map(|(val1, val2)| val1.abs_diff(val2))
        .sum();

    println!("{}", total_sum);
}

pub fn p2() {
    let (list1, list2) = create_input_lists();

    assert_eq!(list1.len(), list2.len());

    let mult_val: u64 = list1
        .iter()
        .map(|elem| list2.iter().filter(|x| **x == *elem).count() as u64 * elem)
        .sum();

    println!("{}", mult_val);
}
