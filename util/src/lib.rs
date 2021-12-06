use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            _ => None,
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}