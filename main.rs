mod util;
use util::{get_nums, get_directions};

// Revision to be less iterative
fn count_increases(nums: &Vec<i64>) -> usize {
  let range = 0..nums.len() - 1;
  // is this really needed - wtf
  let range_array = range.collect::<Vec<usize>>();
  range_array.iter().filter(
    |i|
    nums[**i] < nums[**i + 1] // wtf - how do you rust
    
  ).count()
}

// Revision to be less iterative and more dry
fn count_window_increases(nums: &Vec<i64>) -> usize {
  let range = 0..nums.len() - 2;
  let transformed: Vec<i64>  = range.map(|i|
      nums[i] + nums[i+1] + nums[i+2]
  ).collect::<Vec<i64>>();
  count_increases(&transformed)
}

fn day1() {
  let nums = get_nums("data/nums.txt").unwrap();
  let increases = count_increases(&nums);
  assert!(increases == 1616);
  let window_increases = count_window_increases(&nums);
  assert!(window_increases == 1645);
}

fn location_position(moves: &Vec<(String, i64)>) -> (i64, i64) {
  moves.iter().fold(
    (0, 0),
    |(x_coord, y_coord), (direction, magnitude)|
      match direction.as_str() {
        "forward" => (x_coord + magnitude, y_coord),
        "down" => (x_coord, y_coord + magnitude),
        "up" => (x_coord, y_coord - magnitude),
        _ => (x_coord, y_coord)
      }  
  )
}

fn aimed_location_position(moves: &Vec<(String, i64)>) -> (i64, i64, i64) {
  moves.iter().fold(
    (0, 0, 0),
    |(x_coord, y_coord, aim), (direction, magnitude)|
      match direction.as_str() {
        "forward" => (x_coord + magnitude, y_coord + aim * magnitude, aim),
        "down" => (x_coord, y_coord, aim + magnitude),
        "up" => (x_coord, y_coord, aim - magnitude),
        _ => (x_coord, y_coord, aim)
      }  
  )
}

fn day2() {
  let directions = get_directions("data/directions.txt").unwrap();
  let (x_coord, y_coord) = location_position(&directions);
  println!("POS {} {} {}", x_coord, y_coord, x_coord * y_coord);
  let (x_coord, y_coord, aim) = aimed_location_position(&directions);
  println!("POS {} {} {} {}", x_coord, y_coord, aim, x_coord * y_coord);

}

fn main() {
  day1();
  day2();
  println!("on track!");
}