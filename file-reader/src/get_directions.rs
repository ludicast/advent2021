use util::parse_lines;

pub fn get_directions(filename: &str) -> Vec<(String, i64)> {
    parse_lines(filename, |line| {
        let split = line.split(' ');
        let vec: Vec<&str> = split.collect();
        let direction = String::from(vec[0]);
        let num: i64 = vec[1].parse().unwrap();

        (direction, num)
    })
}
