use util::parse_lines;

#[derive(Debug)]
pub struct SevenSegmentDigit {
    pub chars: Vec<char>,
}

#[derive(Debug)]
pub struct SevenSegmentDisplay {
    pub digits: Vec<SevenSegmentDigit>,
}

pub fn get_seven_segment_display(line: &String) -> SevenSegmentDisplay {
    let split = line.split(" | ");
    let vec: Vec<&str> = split.collect();
    let digits = vec[1]
        .split(' ')
        .map(|digit| SevenSegmentDigit {
            chars: digit.chars().collect(),
        })
        .collect::<Vec<SevenSegmentDigit>>();

    SevenSegmentDisplay { digits }
}

pub fn get_seven_segment_displays(filename: &str) -> Vec<SevenSegmentDisplay> {
    parse_lines(filename, get_seven_segment_display)
}

