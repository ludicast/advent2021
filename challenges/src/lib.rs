mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn run_challenges() {
    day1::day1();
    day2::day2();
    day3::day3();
    day4::day4();
    day5::day5();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
