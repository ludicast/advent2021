use util::display_results;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run_challenges() {
    display_results(1, day1::part1(), day1::part2());
    display_results(2, day2::part1(), day2::part2());
    display_results(3, day3::part1(), day3::part2());
    display_results(4, day4::part1(), day4::part2());
    display_results(5, day5::part1(), day5::part2());
    display_results(6, day6::part1(), day6::part2());
    display_results(7, day7::part1(), day7::part2());
    display_results(8, day8::part1(), day8::part2());
    display_results(9, day9::part1(), day9::part2());
    display_results(10, day10::part1(), day10::part2());
    display_results(11, day11::part1(), day11::part2());
}
