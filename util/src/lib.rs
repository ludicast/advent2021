use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            _ => None,
        })
        .collect::<Vec<String>>()
}

pub fn parse_lines<T>(filename: &str, func: fn(&String) -> T) -> Vec<T> {
    let lines = read_lines(filename);
    lines.iter().map(func).collect()
}

pub fn display_results<T: Debug, U: Debug>(question: i8, part_1: T, part_2: U) {
    println!("Question #{}", question);
    println!("\tPart #1: {:#?}", part_1);
    println!("\tPart #2: {:#?}", part_2);
}
