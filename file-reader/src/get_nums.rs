use util::parse_lines;

pub fn get_nums(filename: &str) -> Vec<i64> {
    parse_lines(filename, |line| line.parse().unwrap())
}
