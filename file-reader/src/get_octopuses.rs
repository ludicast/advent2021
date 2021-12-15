use util::parse_lines;

pub fn get_octopuses(filename: &str) -> [[u32; 10]; 10] {
    let lines = parse_lines(filename, |line: &str| {
        let num_chars = line
            .split("")
            .filter(|&num_char| !num_char.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();
        let mut array_of_nums: [u32; 10] = Default::default();
        array_of_nums[..10].clone_from_slice(&num_chars[..10]);
        array_of_nums
    });
    let mut result: [[u32; 10]; 10] = Default::default();
    result[..10].clone_from_slice(&lines[..10]);
    result
}
