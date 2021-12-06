use util::read_lines;

pub fn get_directions(filename: &str) -> Vec<(String, i64)> {
    let mut res: Vec<(String, i64)> = Vec::new();
    let lines = read_lines(filename);

    for line in lines {
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();
        let direction = String::from(vec[0]);
        let num: i64 = vec[1].parse().unwrap();

        res.push((direction, num));
    }
    return res;
}