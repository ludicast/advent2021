use file_reader::{get_vent_lines, Line, Point};
use std::collections::HashMap;

pub fn count_intersection_points(vent_lines: Vec<Line>, f: &dyn Fn(&Line) -> Vec<Point>) -> i64 {
    let mut point_counter = HashMap::new();
    let points = vent_lines.iter().flat_map(f).collect::<Vec<Point>>();

    for point in points {
        if let Some(value) = point_counter.get(&point.hash()) {
            point_counter.insert(point.hash(), value + 1);
        } else {
            point_counter.insert(point.hash(), 1);
        }
    }

    let values = point_counter.values().cloned().collect::<Vec<i64>>();

    let mut count = 0;
    for value in values {
        if value > 1 {
            count += 1;
        }
    }
    return count;
    /*
    let increases = count_increases(&nums);
    assert!(increases == 1616);
    let window_increases = count_window_increases(&nums);
    assert!(window_increases == 1645);
    */
}

fn get_all_points(line: &Line) -> Vec<Point> {
    line.get_all_points()
}

fn get_straight_points(line: &Line) -> Vec<Point> {
    line.get_non_diagonal_points()
}

pub fn day5() {
    let vent_lines = get_vent_lines("data/vent-lines.txt");
    count_intersection_points(vent_lines, &get_all_points);
    let vent_lines = get_vent_lines("data/vent-lines.txt");
    count_intersection_points(vent_lines, &get_straight_points);
}

pub fn count_all_intersection_points(vent_lines: Vec<Line>) -> i64 {
    count_intersection_points(vent_lines, &get_all_points)
}

pub fn count_straight_intersection_points(vent_lines: Vec<Line>) -> i64 {
    count_intersection_points(vent_lines, &get_straight_points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_straight_vent_lines() {
        let vent_lines = super::get_vent_lines("../fixtures/vent-lines.txt");
        let points = super::count_straight_intersection_points(vent_lines);
        assert_eq!(points, 5);
    }

    #[test]
    fn test_get_all_vent_lines() {
        let vent_lines = super::get_vent_lines("../fixtures/vent-lines.txt");
        let points = super::count_all_intersection_points(vent_lines);
        assert_eq!(points, 12);
    }
}
