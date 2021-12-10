use util::parse_lines;

pub fn get_map(filename: &str) -> Vec<Vec<u8>> {
    parse_lines(filename, |line: &String| {
        let num_chars = line
            .split("")
            .filter(|num_char| *num_char != "")
            .collect::<Vec<&str>>();
        num_chars.iter().map(|num| num.parse().unwrap()).collect()
    })
}
