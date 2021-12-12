use util::parse_lines;

pub fn get_binaries(filename: &str) -> Vec<Vec<u32>> {
    parse_lines(filename, |line| {
        let num_string: String = line.parse().unwrap();
        let num_chars = num_string
            .split("")
            .filter(|&num_char| !num_char.is_empty())
            .collect::<Vec<&str>>();
        num_chars
            .iter()
            .map(|num| if *num == "1" { 1 } else { 0 })
            .collect()
    })
}
