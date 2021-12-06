use file_reader::{get_vent_lines, Point};
use std::collections::HashMap;

pub fn day5() {
    let vent_lines = get_vent_lines("data/vent-lines.txt");
    println!("{:?}", vent_lines);
    let mut point_counter = HashMap::new();
    let points = vent_lines
        .iter()
        .flat_map(|line| line.get_non_diagonal_points())
        .collect::<Vec<Point>>();

    println!("{:?}", points);
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
        if (value > 1) {
            count += 1;
        }
    }
    println!("{:?}", count);
    /*
    let increases = count_increases(&nums);
    assert!(increases == 1616);
    let window_increases = count_window_increases(&nums);
    assert!(window_increases == 1645);
    */
}
