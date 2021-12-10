use std::cmp;

use util::parse_lines;

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
    fn is_diagonal(&self) -> bool {
        self.point1.x != self.point2.x && self.point1.y != self.point2.y
    }

    fn is_horizontal(&self) -> bool {
        self.point1.y == self.point2.y
    }
    fn is_vertical(&self) -> bool {
        self.point1.x == self.point2.x
    }

    fn diagonal_points(&self) -> Vec<Point> {
        let first = cmp::min_by(&self.point1, &self.point2, |pt1, pt2| pt1.x.cmp(&pt2.x));

        let second = cmp::max_by(&self.point1, &self.point2, |pt1, pt2| pt1.x.cmp(&pt2.x));

        let upwards = first.y < second.y;
        (first.x..=second.x)
            .map(|x| {
                let step = x - first.x;
                let y = if upwards {
                    first.y + step
                } else {
                    first.y - step
                };
                Point { x, y }
            })
            .collect()
    }

    fn horizontal_points(&self) -> Vec<Point> {
        let min_x = cmp::min(self.point1.x, self.point2.x);
        let max_x = cmp::max(self.point1.x, self.point2.x);
        (min_x..=max_x)
            .map(|x| Point {
                x,
                y: self.point1.y,
            })
            .collect()
    }

    fn vertical_points(&self) -> Vec<Point> {
        let min_y = cmp::min(self.point1.y, self.point2.y);
        let max_y = cmp::max(self.point1.y, self.point2.y);
        (min_y..=max_y)
            .map(|y| Point {
                x: self.point1.x,
                y,
            })
            .collect()
    }

    pub fn get_non_diagonal_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            return self.horizontal_points();
        }
        if self.is_vertical() {
            return self.vertical_points();
        }

        vec![]
    }

    pub fn get_all_points(&self) -> Vec<Point> {
        if self.is_diagonal() {
            return self.diagonal_points();
        }
        self.get_non_diagonal_points()
    }
}

pub fn set_to_point(point_string: &str) -> Point {
    let split = point_string.split(",");
    let point_nums = split
        .map(|num_as_str| i64::from_str_radix(num_as_str, 10).unwrap())
        .collect::<Vec<i64>>();

    Point {
        x: point_nums[0],
        y: point_nums[1],
    }
}

pub fn get_vent_lines(filename: &str) -> Vec<Line> {
    parse_lines(filename, |line| {
        let split = line.split(" -> ");

        let start_end: Vec<&str> = split.collect();
        let point1 = set_to_point(start_end[0]);
        let point2 = set_to_point(start_end[1]);
        Line { point1, point2 }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
