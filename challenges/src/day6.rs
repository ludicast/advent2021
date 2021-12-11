use file_reader::get_lanterns;

fn calculate_lanternfish(mut lanterns: [i64; 9], days: usize) -> i64 {
    for _ in 0..days {
        let spawners = lanterns[0];
        for i in 0..8 {
            lanterns[i] = lanterns[i + 1];
        }
        lanterns[6] += spawners;
        lanterns[8] = spawners;
    }
    lanterns.to_vec().iter().sum()
}

pub fn part1() -> i64 {
    let lanterns = get_lanterns("./data/lanterns.txt");
    let sum = calculate_lanternfish(lanterns, 80);
    assert_eq!(sum, 362346);
    sum
}

pub fn part2() -> i64 {
    let lanterns = get_lanterns("./data/lanterns.txt");
    let sum = calculate_lanternfish(lanterns, 256);
    assert_eq!(sum, 1639643057051);
    sum
}
#[cfg(test)]
mod tests {
    #[test]
    fn calculate_lanternfish_after_80() {
        let lanterns = super::get_lanterns("../fixtures/lanterns.txt");
        let count = super::calculate_lanternfish(lanterns, 80);
        assert_eq!(count, 5934);
    }

    #[test]
    fn calculate_lanternfish_after_256() {
        let lanterns = super::get_lanterns("../fixtures/lanterns.txt");
        let count = super::calculate_lanternfish(lanterns, 256);
        assert_eq!(count, 26984457539);
    }
}
