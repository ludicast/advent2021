use std::fs::File;
use std::io::{self, BufRead};

pub fn get_nums(filename: &str) -> io::Result<Vec<i64>> {
  let mut nums: Vec<i64> = Vec::new();
  let file = File::open(filename)?;
  let lines = io::BufReader::new(file).lines();
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
  let file = File::open(filename)?;
  let lines = io::BufReader::new(file).lines();

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