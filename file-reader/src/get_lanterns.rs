use util::read_lines;

pub fn get_lanterns(filename: &str) -> [i64 ; 9] {
    let mut nums: [i64; 9] = [0; 9];
    let lines = read_lines(filename);
    let parsed_nums = lines[0].split(",").map(
        |num_string| i64::from_str_radix(num_string, 10).unwrap()
    ).collect::<Vec<i64>>();
    for num in parsed_nums {
        nums[num as usize] += 1;
    }
    return nums;
}
