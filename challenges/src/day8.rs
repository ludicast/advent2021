use std::collections::HashSet;

use file_reader::{get_seven_segment_displays, SevenSegmentDisplay};

fn calculate_inputs(display: &SevenSegmentDisplay) {
    /*
    let mut definitions: [HashSet<char>; 10] = Default::default();

    let mut unknown = vec![];
    for possible in display.input.iter() {
        let chars = &possible.chars;
        let char_set = HashSet::from_iter(chars.clone());
        match chars.len() {
            2 => {
               definitions[1] = char_set;
            },
            3 => {
               definitions[7] = char_set;
            },
            4 => {
               definitions[4] = char_set;
            },
            7 => {
               definitions[8] = char_set;
            },
            _ => {
                unknown.push(possible);
            }
        }
    }
    println!("{:#?}", definitions);
    unknown.retain(|obj| obj.chars.len() != 6);
    println!("{:#?}", unknown);
    1 4 7 8

    5 chars
    3 (contains 1)
    2 (not ccntained by 9)
    5 (not 3 contained by 9)

    6 chars
    ********
    9 (contains 3) (contains intersection of 4 and 7)
    0 (contains 7, does not contain 3)
    6 (contains 5)
    let three = unknown.iter().find(|i| {
        let chars = i.chars;
        let hash_set = HashSet<char>::from_iter(i.chars.clone());
        true
    }).unwrap();

    */
    // let four_and_seven = definitions[4].union(&definitions[7]); //.collect::<HashSet<char>>();

    // .union(&definitions[3]);
    /*
        for opt in unknown {

            if opt.chars.len() == 5 {
               println!("foive");
            }

        }
    */
}

fn count_unique_numbers(displays: Vec<SevenSegmentDisplay>) -> usize {
    displays
        .iter()
        .flat_map(|display| &display.output)
        .filter(|digit| [2_usize, 3_usize, 4_usize, 7_usize].contains(&digit.len()))
        .count()
}

pub fn part1() -> usize {
    let displays = get_seven_segment_displays("./data/seven-segment-displays.txt");
    let unique_numbers = count_unique_numbers(displays);
    assert_eq!(unique_numbers, 383);
    unique_numbers
}

pub fn part2() -> usize {
    let displays = get_seven_segment_displays("./data/seven-segment-displays.txt");
    // calculate_inputs(&displays[0]);
    let unique_numbers = count_unique_numbers(displays);
    assert_eq!(unique_numbers, 383);
    unique_numbers
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
    fn test_calculate_inputs() {
        let displays = super::get_seven_segment_displays("../fixtures/seven-segment-displays.txt");
        super::calculate_inputs(&displays[0]);
    }
}
