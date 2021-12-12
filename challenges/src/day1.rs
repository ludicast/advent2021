use file_reader::get_nums;

// Revision to be less iterative
fn count_increases(nums: &[i64]) -> usize {
    let range = 0..nums.len() - 1;
    range.filter(|&i| nums[i] < nums[i + 1]).count()
}

// Revision to be less iterative and more dry
fn count_window_increases(nums: &[i64]) -> usize {
    let transformed = nums
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i64>>();
    count_increases(&transformed)
}

pub fn part1() -> usize {
    let nums = get_nums("data/nums.txt");
    let increases = count_increases(&nums);
    assert!(increases == 1616);
    increases
}

pub fn part2() -> usize {
    let nums = get_nums("data/nums.txt");
    let window_increases = count_window_increases(&nums);
    assert!(window_increases == 1645);
    window_increases
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
