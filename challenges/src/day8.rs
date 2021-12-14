use std::collections::HashSet;

use file_reader::{get_seven_segment_displays, SevenSegmentDisplay};

fn calculate_inputs(display: &SevenSegmentDisplay) -> [HashSet<char>; 10] {
    let mut definitions: [HashSet<char>; 10] = Default::default();

    let mut unknown = vec![];
    for chars in display.input.iter() {
        match chars.len() {
            2 => {
                definitions[1] = chars.clone();
            }
            3 => {
                definitions[7] = chars.clone();
            }
            4 => {
                definitions[4] = chars.clone();
            }
            7 => {
                definitions[8] = chars.clone();
            }
            _ => {
                unknown.push(chars);
            }
        }
    }

    let &three = unknown
        .iter()
        .find(|&&i| i.len() == 5 && i.is_superset(&definitions[1]))
        .unwrap();
    definitions[3] = three.clone();
    let &nine = unknown
        .iter()
        .find(|&&i| i.len() == 6 && i.is_superset(&definitions[3]))
        .unwrap();
    definitions[9] = nine.clone();
    let &five = unknown
        .iter()
        .find(|&&i| i.len() == 5 && i.is_subset(&definitions[9]) && !i.is_superset(&definitions[1]))
        .unwrap();
    definitions[5] = five.clone();
    let &two = unknown
        .iter()
        .find(|&&i| i.len() == 5 && !i.is_subset(&definitions[9]))
        .unwrap();
    definitions[2] = two.clone();
    let &six = unknown
        .iter()
        .find(|&&i| i.len() == 6 && !i.is_superset(&definitions[1]))
        .unwrap();
    definitions[6] = six.clone();
    let &zero = unknown
        .iter()
        .find(|&&i| i.len() == 6 && definitions.iter().all(|definition| definition != i))
        .unwrap();
    definitions[0] = zero.clone();
    definitions
}

fn get_value(inputs: &[HashSet<char>; 10], digit: &HashSet<char>) -> usize {
    (0..10)
        .find(|&idx| inputs[idx].is_subset(digit) && inputs[idx].is_superset(digit))
        .unwrap()
}
fn count_unique_numbers(displays: Vec<SevenSegmentDisplay>) -> usize {
    let digit_map = displays
        .iter()
        .map(|display| {
            let inputs = &calculate_inputs(display);
            display
                .output
                .iter()
                .map(|digit| get_value(inputs, digit))
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();
    digit_map
        .iter()
        .flatten()
        .filter(|i| [1_usize, 4_usize, 7_usize, 8_usize].contains(i))
        .count()
}

fn sum_numbers(displays: Vec<SevenSegmentDisplay>) -> i32 {
    let vals = displays.iter().map(|display| {
        let inputs = &calculate_inputs(display);
        let digits = display
            .output
            .iter()
            .map(|digit| get_value(inputs, digit))
            .collect::<Vec<usize>>();
        let res = digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];
        res as i32
    });
    vals.sum::<i32>()
}

pub fn part1() -> usize {
    let displays = get_seven_segment_displays("./data/seven-segment-displays.txt");
    let unique_numbers = count_unique_numbers(displays);
    assert_eq!(unique_numbers, 383);
    unique_numbers
}

pub fn part2() -> i32 {
    let displays = get_seven_segment_displays("./data/seven-segment-displays.txt");
    let summed_numbers = sum_numbers(displays);
    assert_eq!(summed_numbers, 998900);
    summed_numbers
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_unique_numbers() {
        let displays = super::get_seven_segment_displays("../fixtures/seven-segment-displays.txt");
        let unique_numbers = super::count_unique_numbers(displays);
        assert_eq!(unique_numbers, 26);
    }

    #[test]
    fn test_sum_numbers() {
        let displays = super::get_seven_segment_displays("../fixtures/seven-segment-displays.txt");
        let summed_numbers = super::sum_numbers(displays);
        assert_eq!(summed_numbers, 61229);
    }

    #[test]
    fn test_calculate_inputs() {
        let displays = super::get_seven_segment_displays("../fixtures/seven-segment-displays.txt");
        super::calculate_inputs(&displays[0]);
    }
}
