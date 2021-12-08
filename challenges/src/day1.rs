use file_reader::get_nums;
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
    let nums = get_nums("data/nums.txt");
    let increases = count_increases(&nums);
    assert!(increases == 1616);
    let window_increases = count_window_increases(&nums);
    assert!(window_increases == 1645);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_increases() {
        let nums = super::get_nums("../fixtures/nums.txt");
        let increases = super::count_increases(&nums);
        assert_eq!(increases, 7);
    }

    #[test]
    fn test_count_window_increases() {
        let nums = super::get_nums("../fixtures/nums.txt");
        let increases = super::count_window_increases(&nums);
        assert_eq!(increases, 5);
    }
}
