use util::read_lines;

pub fn get_nums(filename: &str) -> Vec<i64> {
    let lines = read_lines(filename);
    lines.iter().map(|line|
        line.parse().unwrap()
    ).collect()
}
