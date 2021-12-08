use util::read_lines;

pub fn get_directions(filename: &str) -> Vec<(String, i64)> {
    let lines = read_lines(filename);

    lines
        .iter()
        .map(|line| {
            let split = line.split(" ");
            let vec: Vec<&str> = split.collect();
            let direction = String::from(vec[0]);
            let num: i64 = vec[1].parse().unwrap();

            (direction, num)
        })
        .collect()
}
