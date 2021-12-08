use util::read_lines;

pub struct SevenSegmentDigit {
    pub chars: Vec<char>,
}
pub struct SevenSegmentDisplay {
    pub digits: Vec<SevenSegmentDigit>,
}

pub fn get_seven_segment_displays(filename: &str) -> Vec<SevenSegmentDisplay> {
    let lines = read_lines(filename);

    lines
        .iter()
        .map(|line| {
            let split = line.split(" | ");
            let vec: Vec<&str> = split.collect();
            let signal_pattern_strings = String::from(vec[0]);
            let digits = vec[1]
                .split(" ")
                .map(|digit| SevenSegmentDigit {
                    chars: digit.chars().collect(),
                })
                .collect::<Vec<SevenSegmentDigit>>();

            SevenSegmentDisplay { digits }
        })
        .collect::<Vec<SevenSegmentDisplay>>()
}
