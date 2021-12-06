use file_reader::{get_lanterns};

fn calculate_lanternfish(mut lanterns: [i64; 9], days: usize) -> i64 {
    for _ in 0..days {
        let spawners = lanterns[0];
        for i in 0..8 {
            lanterns[i] = lanterns[i+1];
        }
        lanterns[6] += spawners;
        lanterns[8] = spawners;
    }
    lanterns.to_vec().iter().sum()
}

pub fn day6() {
    let mut lanterns = get_lanterns("./data/lanterns.txt");
    let sum = calculate_lanternfish(lanterns, 256);
    println!("flts {:?}, sum {}", lanterns, sum);

}