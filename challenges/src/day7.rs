use file_reader::get_parsed_nums;
use std::cmp;

fn total_distance(crabs: &Vec<i64>, point: i64) -> i64 {
    crabs
        .iter()
        .fold(0, |total, crab| total + (crab - point).abs())
}

fn magnified_total_distance(crabs: &Vec<i64>, point: i64) -> i64 {
    crabs.iter().fold(0, |total, crab| {
        let dist = (crab - point).abs();
        total + ((dist + 1) * dist) / 2
    })
}

fn calculate_min_fuel(crabs: &Vec<i64>, magnitude: fn(crabs: &Vec<i64>, point: i64) -> i64) -> i64 {
    let largest = crabs
        .iter()
        .fold(crabs[0], |max, crab| cmp::max(max, *crab));
    let mut min_fuel = magnitude(&crabs, 0);

    for i in 1..=largest {
        let dist = magnitude(&crabs, i);
        if dist < min_fuel {
            min_fuel = dist;
        }
    }
    return min_fuel;
}

pub fn part1() -> i64 {
    let crabs = get_parsed_nums("./data/crab-locations.txt");
    let min_fuel = calculate_min_fuel(&crabs, total_distance);
    assert_eq!(min_fuel, 345035);
    min_fuel
}

pub fn part2() -> i64 {
    let crabs = get_parsed_nums("./data/crab-locations.txt");
    let min_fuel = calculate_min_fuel(&crabs, magnified_total_distance);
    assert_eq!(min_fuel, 97038163);
    return min_fuel;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_fuel_via_total_distance() {
        let crabs = super::get_parsed_nums("../fixtures/crab-locations.txt");
        let min_fuel = super::calculate_min_fuel(&crabs, super::total_distance);
        assert_eq!(min_fuel, 37);
    }

    #[test]
    fn test_min_fuel_via_magnified_total_distance() {
        let crabs = super::get_parsed_nums("../fixtures/crab-locations.txt");
        let min_fuel = super::calculate_min_fuel(&crabs, super::magnified_total_distance);
        assert_eq!(min_fuel, 168);
    }
}
