use file_reader::{get_seven_segment_displays, SevenSegmentDisplay};
use std::{cmp, fmt::Display};

fn display_to_digits(display: &SevenSegmentDisplay) -> Vec<Vec<char>> {
    display
        .digits
        .iter()
        .map(|digit| digit.chars.clone())
        .collect()
}

fn count_unique_numbers() {
    let displays = get_seven_segment_displays("./data/seven-segment-displays.txt");

    println!(" .  count: {}", displays.len());
    let unique_digits: Vec<Vec<char>> = displays
        .iter()
        .flat_map(|display| display_to_digits(display))
        .filter(|digit| [2 as usize, 3 as usize, 4 as usize, 7 as usize].contains(&digit.len()))
        .collect();
    println!(" .  count: {}", unique_digits.len());
}

pub fn day8() {
    count_unique_numbers();
}
