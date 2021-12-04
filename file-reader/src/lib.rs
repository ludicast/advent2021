use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            _ => None,
        })
        .collect::<Vec<String>>()
}

pub fn get_nums(filename: &str) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();
    let lines = get_lines(filename);

    for line in lines {
        let num: i64 = line.parse().unwrap();
        nums.push(num);
    }
    return nums;
}

pub fn get_directions(filename: &str) -> Vec<(String, i64)> {
    let mut res: Vec<(String, i64)> = Vec::new();
    let lines = get_lines(filename);

    for line in lines {
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();
        let direction = String::from(vec[0]);
        let num: i64 = vec[1].parse().unwrap();

        res.push((direction, num));
    }
    return res;
}

pub struct BingoGame {
    pub balls: Vec<i64>,
}

pub fn get_bingos(filename: &str) -> BingoGame {
    let lines = get_lines(filename);
    let balls = lines[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let games_count = (lines.len() - 1) / 6;

    let games: Vec<Vec<Vec<i64>>> = (1..=games_count).map(|game_num| {
        vec![vec![1,2,3]]
    }
    ).collect::<Vec<Vec<Vec<i64>>>>();
    BingoGame {
        balls
    }
}

pub fn get_binaries(filename: &str) -> Vec<Vec<u32>> {
    let mut nums: Vec<Vec<u32>> = Vec::new();
    let lines = get_lines(filename);

    for line in lines {
        let num_string: String = line.parse().unwrap();
        let num_chars = num_string
            .split("")
            .filter(|num_char| *num_char != "")
            .collect::<Vec<&str>>();
        nums.push(
            num_chars
                .iter()
                .map(|num| if *num == "1" { 1 } else { 0 })
                .collect(),
        );
    }
    return nums;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
