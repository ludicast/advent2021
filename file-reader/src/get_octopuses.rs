use util::parse_lines;

pub fn get_octopuses(filename: &str) -> [[u8; 10]; 10] {
    let lines = parse_lines(filename, |line: &str| {
        let num_chars = line
            .split("")
            .filter(|&num_char| !num_char.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>();
        let mut array_of_nums: [u8; 10] = Default::default();
        for i in 0..10 {
            array_of_nums[i] = num_chars[i];
        }
        array_of_nums
    });
    let mut result: [[u8; 10]; 10] = Default::default();
    result[..10].clone_from_slice(&lines[..10]);
    result
}
