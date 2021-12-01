use std::fs::File;
use std::io::{self, BufRead};

pub fn get_nums(filename: &str) -> io::Result<Vec<i64>> {
  let mut nums: Vec<i64> = Vec::new();
  let file = File::open(filename)?;
  let lines = io::BufReader::new(file).lines();
  for line in lines {
    if let Ok(str) = line {
      let num: i64 = str.parse().unwrap();
      nums.push(num);
    }
  }
  return Ok(nums);
}
