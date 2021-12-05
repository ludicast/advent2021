pub mod read_lines;
use read_lines::read_lines;

use std::cmp;
use std::collections::HashSet;

pub fn get_nums(filename: &str) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();
    let lines = read_lines(filename);

    for line in lines {
        let num: i64 = line.parse().unwrap();
        nums.push(num);
    }
    return nums;
}

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

#[derive(PartialEq)]
pub struct Board {
    pub values: Vec<Vec<i64>>,
    pub value_set: HashSet<i64>,
}

impl Board {
    pub fn new(values: Vec<Vec<i64>>) -> Self {
        let mut value_set = HashSet::new();
        for row in &values {
            for cell in row {
                value_set.insert(*cell);
            }
        }
        Board { values, value_set }
    }

    fn columns(&self) -> Vec<Vec<i64>> {
        let column_count = self.values.len();
        (0..column_count)
            .map(|col_num| {
                self.values
                    .iter()
                    .map(|row| row[col_num])
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>()
    }

    pub fn has_bingo(&self, nums: &mut HashSet<i64>) -> bool {
        for row in &self.values {
            if row.iter().all(|item| nums.contains(item)) {
                return true;
            }
        }
        for col in self.columns() {
            if col.iter().all(|item| nums.contains(item)) {
                return true;
            }
        }
        false
    }

    pub fn matched(&self, nums: &mut HashSet<i64>) -> (Vec<i64>, Vec<i64>) {
        let values = self.values.clone();

        let matched_cells = values
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|cell| nums.contains(cell))
                    .map(|cell| *cell)
            })
            .collect::<Vec<i64>>();
        let non_matched_cells = values
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|cell| !nums.contains(cell))
                    .map(|cell| *cell)
            })
            .collect::<Vec<i64>>();
        (matched_cells, non_matched_cells)
    }
}

pub struct BingoGame {
    pub balls: Vec<i64>,
    pub boards: Vec<Board>,
}

pub fn get_bingos(filename: &str) -> BingoGame {
    let lines = read_lines(filename);
    let balls = lines[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let games_count = (lines.len() - 1) / 6;

    let boards: Vec<Board> = (0..games_count)
        .map(|game_num| {
            let start = 2 + game_num * 6;
            let values = (start..start + 5)
                .map(|row_num| {
                    lines[row_num]
                        .split(" ")
                        // some spots are double-spaced, but regex is a 3rd party util
                        .filter(|piece| *piece != "")
                        .map(|num_string| i64::from_str_radix(num_string, 10).unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();
            Board::new(values)
        })
        .collect::<Vec<Board>>();

    BingoGame { balls, boards }
}

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

#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn hash(&self) -> String {
        format!("{} {}", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Line {
    pub point1: Point,
    pub point2: Point,
}
impl Line {
    
    fn is_horizontal(&self) -> bool {
        self.point1.y == self.point2.y
    }
    fn is_vertical(&self) -> bool {
        self.point1.x == self.point2.x
    }
    fn horizontal_points(&self) -> Vec<Point>{
        let min_x = cmp::min(self.point1.x, self.point2.x);
        let max_x = cmp::max(self.point1.x, self.point2.x);
        (min_x..=max_x).map(|x|
            Point{
                x,
                y: self.point1.y
            }
        ).collect()
    }
    fn vertical_points(&self) -> Vec<Point>{
        let min_y= cmp::min(self.point1.y, self.point2.y);
        let max_y = cmp::max(self.point1.y, self.point2.y);
        (min_y..=max_y).map(|y|
            Point{
                x: self.point1.x,
                y
            }
        ).collect()
    }
    pub fn get_non_diagonal_points(&self) -> Vec<Point>{
        if self.is_horizontal() {
            return self.horizontal_points()
        }
        if self.is_vertical() {
            return self.vertical_points()
        }

        vec![]
    }


}


pub fn set_to_point(point_string: &str) -> Point {
    let split = point_string.split(",");
    let point_nums = split.map(|num_as_str|{
        i64::from_str_radix(num_as_str, 10).unwrap()
    }).collect::<Vec<i64>>();

    Point {
        x: point_nums[0],
        y: point_nums[1],
    }
}

pub fn get_vent_lines(filename: &str) -> Vec<Line> {
    let mut vent_lines: Vec<Line> = Vec::new();
    let lines = read_lines(filename);

    for line in lines {
        let split = line.split(" -> ");

        let start_end: Vec<&str> = split.collect();
        let point1 = set_to_point(start_end[0]);
        let point2 = set_to_point(start_end[1]);
        vent_lines.push(Line {
            point1, point2
        })
    }
    return vent_lines;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
