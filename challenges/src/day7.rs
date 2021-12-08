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

pub fn day7() {
    let crabs = get_parsed_nums("./data/crab-locations.txt");

    let largest = crabs
        .iter()
        .fold(crabs[0], |max, crab| cmp::max(max, *crab));
    println!("{}", largest);
    for i in 0..=largest {
        let dist = magnified_total_distance(&crabs, i);
        println!(" {} / {} ", dist, i);
    }
}
