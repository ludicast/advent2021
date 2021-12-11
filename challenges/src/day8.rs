use file_reader::{get_seven_segment_display, get_seven_segment_displays, SevenSegmentDisplay};

fn display_to_digits(display: &SevenSegmentDisplay) -> Vec<Vec<char>> {
    display
        .digits
        .iter()
        .map(|digit| digit.chars.clone())
        .collect()
}

fn count_unique_numbers(displays: Vec<SevenSegmentDisplay>) -> usize {
    let unique_digits: Vec<Vec<char>> = displays
        .iter()
        .flat_map(|display| display_to_digits(display))
        .filter(|digit| [2 as usize, 3 as usize, 4 as usize, 7 as usize].contains(&digit.len()))
        .collect();
    unique_digits.len()
}

pub fn part1() -> usize {
    let displays = get_seven_segment_displays("./data/seven-segment-displays.txt");
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
    fn test_get_unique_value() {
        let words = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let display = super::get_seven_segment_display(&String::from(words));
        let digits = super::display_to_digits(&display);
        assert_eq!(digits.len(), 4);
        assert_eq!(digits[0], "fdgacbe".chars().collect::<Vec<char>>());
    }
}
