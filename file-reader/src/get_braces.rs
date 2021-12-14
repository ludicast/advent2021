use util::parse_lines;

pub fn get_braces(filename: &str) -> Vec<Vec<char>> {
    parse_lines(filename, |line: &str| {
        let char_chars = line
            .split("")
            .filter(|&num_char| !num_char.is_empty())
            .collect::<Vec<&str>>();
        char_chars.iter().map(|num| num.parse().unwrap()).collect()
    })
}
