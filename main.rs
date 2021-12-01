mod util;
use util::get_nums;

fn count_increases(nums: &Vec<i64>) -> usize {
  let range = 0..nums.len() - 1;
  let range_array = range.collect::<Vec<usize>>();
  range_array.iter().filter(
    |i|
    nums[**i] < nums[**i + 1] 
    
  ).count()
}

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

fn main() {
  day1();
  println!("on track!");
}