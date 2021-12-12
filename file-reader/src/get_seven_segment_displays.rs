use util::parse_lines;

#[derive(Debug)]
pub struct SevenSegmentDigit {
    pub chars: Vec<char>,
}

impl SevenSegmentDigit {
    fn bulk_create(rep: &str) -> Vec<Self> {
        rep.split(' ')
        .map(|digit| Self {
            chars: digit.chars().collect(),
        })
        .collect::<Vec<Self>>()
    }
}

#[derive(Debug)]
pub struct SevenSegmentDisplay {
    pub output: Vec<SevenSegmentDigit>,
    pub input: Vec<SevenSegmentDigit>,
}

pub fn get_seven_segment_display(line: &str) -> SevenSegmentDisplay {
    let split = line.split(" | ");
    let vec: Vec<&str> = split.collect();
    let input = SevenSegmentDigit::bulk_create(vec[0]);
    let output = SevenSegmentDigit::bulk_create(vec[1]);

    SevenSegmentDisplay { input, output }
}

pub fn get_seven_segment_displays(filename: &str) -> Vec<SevenSegmentDisplay> {
    parse_lines(filename, get_seven_segment_display)
}

