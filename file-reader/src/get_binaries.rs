use util::read_lines;

pub fn get_binaries(filename: &str) -> Vec<Vec<u32>> {
    let mut nums: Vec<Vec<u32>> = Vec::new();
    let lines = read_lines(filename);

    for line in lines {
        let num_string: String = line.parse().unwrap();
        let num_chars = num_string
            .split("")
            .filter(|num_char| *num_char != "")
            .collect::<Vec<&str>>();
        nums.push(
            num_chars
                .iter()
                .map(|num| if *num == "1" { 1 } else { 0 })
                .collect(),
        );
    }
    return nums;
}