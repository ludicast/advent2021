use util::read_lines;

pub fn get_parsed_nums(filename: &str) -> Vec<i64> {
    let lines = read_lines(filename);
    lines[0]
        .split(",")
        .map(|num_string| i64::from_str_radix(num_string, 10).unwrap())
        .collect::<Vec<i64>>()
}

pub fn get_lanterns(filename: &str) -> [i64; 9] {
    let mut nums: [i64; 9] = [0; 9];
    let parsed_nums = get_parsed_nums(filename);
    for num in parsed_nums {
        nums[num as usize] += 1;
    }
    return nums;
}
