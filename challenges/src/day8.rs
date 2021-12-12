use file_reader::{get_seven_segment_displays, SevenSegmentDisplay};

fn display_to_digits(display: &SevenSegmentDisplay) -> Vec<Vec<char>> {
    display
        .digits
        .iter()
        .map(|digit| digit.chars.clone())
        .collect()
}

fn count_unique_numbers(displays: Vec<SevenSegmentDisplay>) -> usize {
    displays
        .iter()
        .flat_map(|display| display_to_digits(display))
        .filter(|digit| [2_usize, 3_usize, 4_usize, 7_usize].contains(&digit.len()))
        .count()
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
}
