use std::collections::HashSet;
use util::parse_lines;

#[derive(Debug)]
pub struct SevenSegmentDisplay {
    pub output: Vec<HashSet<char>>,
    pub input: Vec<HashSet<char>>,
}

fn bulk_create(rep: &str) -> Vec<HashSet<char>> {
    rep.split(' ')
        .map(|digit| HashSet::from_iter(digit.chars().clone()))
        .collect()
}

pub fn get_seven_segment_display(line: &str) -> SevenSegmentDisplay {
    let split = line.split(" | ");
    let vec: Vec<&str> = split.collect();
    let input = bulk_create(vec[0]);
    let output = bulk_create(vec[1]);

    SevenSegmentDisplay { input, output }
}

pub fn get_seven_segment_displays(filename: &str) -> Vec<SevenSegmentDisplay> {
    parse_lines(filename, get_seven_segment_display)
}
