use util::read_lines;

pub fn get_nums(filename: &str) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();
    let lines = read_lines(filename);

    for line in lines {
        let num: i64 = line.parse().unwrap();
        nums.push(num);
    }
    return nums;
}
