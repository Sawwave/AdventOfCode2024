use regex::Regex;
use std::{fs::read_to_string, path::Path};

fn read_file_blob(file_src: &str) -> String {
    let file_contents = read_to_string(Path::new(file_src)).expect("could not read file blob");
    return file_contents;
}

pub fn p1() {
    let file_src = "inputs/day3.txt";
    let file_contents = read_file_blob(file_src);
    let mult_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("unable to generate regex");

    let mut accumulator = 0;
    for (_, [operand_a, operand_b]) in mult_regex
        .captures_iter(&file_contents)
        .map(|content| content.extract())
    {
        let operand_a_int: u64 = operand_a.parse().expect("unable to parse first operand");
        let operand_b_int: u64 = operand_b.parse().expect("unable to parse second operand");

        accumulator += operand_a_int * operand_b_int;
    }

    println!("{}", accumulator);
}

pub fn p2() {
    let file_src = "inputs/day3.txt";
    let file_contents = read_file_blob(file_src);
    let mult_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("unable to generate regex");
    Regex::new(r"(?:^|do\(\))(.*?)(?:$|don't\(\))").expect("unable to make enable regex");
    let do_regex = Regex::new(r"do\(\)|^").expect("could not make do regex");
    let dont_regex = Regex::new(r"don't\(\)|$").expect("could not make dont regex");

    //regex that matches "do()", "dont()", or "mul(a,b)"
    let element_regex = Regex::new(r"(do\(\)|don\'t\(\)|mul\(\d+,\d+\))").unwrap();

    let mut accumulator: u64 = 0;
    let mut enabled = true;

    for m in element_regex.captures_iter(&file_contents) {
        let element = m.get(0).unwrap().as_str();
        if element == "do()" {
            enabled = true;
        } else if element == "don't()" {
            enabled = false;
        } else if enabled{
            let operands: (&str, [&str; 2]) = mult_regex.captures(element).unwrap().extract();
            accumulator += operands.1[0].parse::<u64>().unwrap() * operands.1[1].parse::<u64>().unwrap();
        }
    }
    print!("{}", accumulator);
}
