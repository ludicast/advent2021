use file_reader::{get_binaries, get_directions, get_nums};

// Revision to be less iterative
fn count_increases(nums: &Vec<i64>) -> usize {
    let range = 0..nums.len() - 1;
    // is this really needed - wtf
    let range_array = range.collect::<Vec<usize>>();
    range_array
        .iter()
        .filter(
            |i| nums[**i] < nums[**i + 1], // wtf - how do you rust
        )
        .count()
}

// Revision to be less iterative and more dry
fn count_window_increases(nums: &Vec<i64>) -> usize {
    let range = 0..nums.len() - 2;
    let transformed: Vec<i64> = range
        .map(|i| nums[i] + nums[i + 1] + nums[i + 2])
        .collect::<Vec<i64>>();
    count_increases(&transformed)
}

pub fn day1() {
    let nums = get_nums("data/nums.txt").unwrap();
    let increases = count_increases(&nums);
    assert!(increases == 1616);
    let window_increases = count_window_increases(&nums);
    assert!(window_increases == 1645);
}

fn location_position(moves: &Vec<(String, i64)>) -> (i64, i64) {
    moves.iter().fold(
        (0, 0),
        |(x_coord, y_coord), (direction, magnitude)| match direction.as_str() {
            "forward" => (x_coord + magnitude, y_coord),
            "down" => (x_coord, y_coord + magnitude),
            "up" => (x_coord, y_coord - magnitude),
            _ => (x_coord, y_coord),
        },
    )
}

fn aimed_location_position(moves: &Vec<(String, i64)>) -> (i64, i64, i64) {
    moves.iter().fold(
        (0, 0, 0),
        |(x_coord, y_coord, aim), (direction, magnitude)| match direction.as_str() {
            "forward" => (x_coord + magnitude, y_coord + aim * magnitude, aim),
            "down" => (x_coord, y_coord, aim + magnitude),
            "up" => (x_coord, y_coord, aim - magnitude),
            _ => (x_coord, y_coord, aim),
        },
    )
}

pub fn day2() {
    let directions = get_directions("data/directions.txt").unwrap();
    let (x_coord, y_coord) = location_position(&directions);
    println!("POS {} {} {}", x_coord, y_coord, x_coord * y_coord);
    let (x_coord, y_coord, aim) = aimed_location_position(&directions);
    println!("POS {} {} {} {}", x_coord, y_coord, aim, x_coord * y_coord);
}

fn most_common(nums: &Vec<Vec<u32>>, col_num: usize, gamma: bool) -> u32 {
  let rows: u32= nums.len().try_into().unwrap();
  let columns = nums[0].len();
  let low_floor = rows / 2;
  let floor = rows - low_floor;
  let count = nums.iter().fold(0, |count, row| count + row[col_num]);
  if count >= floor {
    if gamma {1} else {0}
  } else {
    if gamma {0} else {1}
  }
}

fn generate_new_num(nums: &Vec<Vec<u32>>, gamma: bool) -> u64 {
    let rows = nums.len();
    let columns = nums[0].len();
    let mut bit_string = String::from("");
    for col_num in 0..columns {
        let bit_int = most_common(&nums, col_num, gamma);
        let bit_char = match bit_int {
          1 => '1',
          _ => '0'
        };
        bit_string.push(bit_char);
    }
    return u64::from_str_radix(bit_string.as_str(), 2).unwrap();
}


fn generate_winning_num(nums: &Vec<Vec<u32>>, gamma: bool) -> u64 {
    let rows = nums.len();
    let columns = nums[0].len();
    // let mut bit_string = String::from("");
    let mut nums: Vec<Vec<u32>> = nums.clone();
    for col_num in 0..columns {
        let common = most_common(&nums, col_num, gamma);
        nums.retain(
            |row| row[col_num] == common
        );
        if nums.len() == 1 {
          break
        }
    }
    let bit_chars: Vec<&str> = nums[0].iter().map(
      |num| match num {
        1 => "1",
        0 => "0",
        _ => panic!("unknown {}", num)
      }
    ).collect::<Vec<&str>>();
    let bit_string = bit_chars.join("");
    return u64::from_str_radix(bit_string.as_str(), 2).unwrap();
}

pub fn day3() {
    let binaries = get_binaries("data/binaries.txt").unwrap();
    let new_num = generate_new_num(&binaries, true);
    let new_num_reverse = generate_new_num(&binaries, false);
    println!("{:?}", new_num * new_num_reverse);
    let new_num = generate_winning_num(&binaries, true);
    let new_num_reverse = generate_winning_num(&binaries, false);
    println!("{:?}", new_num * new_num_reverse);
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
