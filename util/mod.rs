use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(filename: &str) -> io::Result<std::io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_nums(filename: &str) -> io::Result<Vec<i64>> {
    let mut nums: Vec<i64> = Vec::new();
    let lines = get_lines(filename)?;

    for line in lines {
        if let Ok(line) = line {
            let num: i64 = line.parse().unwrap();
            nums.push(num);
        }
    }
    return Ok(nums);
}

pub fn get_directions(filename: &str) -> io::Result<Vec<(String, i64)>> {
    let mut res: Vec<(String, i64)> = Vec::new();
    let lines = get_lines(filename)?;

    for line in lines {
        if let Ok(line) = line {
            let split = line.split(" ");
            let vec: Vec<&str> = split.collect();
            let direction = String::from(vec[0]);
            let num: i64 = vec[1].parse().unwrap();

            res.push((direction, num));
        }
    }
    return Ok(res);
}

pub fn get_binaries(filename: &str) -> io::Result<Vec<Vec<u32>>> {
    let mut nums: Vec<Vec<u32>> = Vec::new();
    let lines = get_lines(filename)?;

    for line in lines {
        if let Ok(line) = line {
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
    }
    return Ok(nums);
}
