use file_reader::get_binaries;

fn most_common(nums: &[Vec<u32>], col_num: usize, gamma: bool) -> u32 {
    let rows: u32 = nums.len().try_into().unwrap();
    let low_floor = rows / 2;
    let floor = rows - low_floor;
    let count = nums.iter().fold(0, |count, row| count + row[col_num]);
    if count >= floor {
        if gamma {
            1
        } else {
            0
        }
    } else if gamma {
        0
    } else {
        1
    }
}

fn generate_new_num(nums: &[Vec<u32>], gamma: bool) -> u64 {
    let columns = nums[0].len();
    let mut bit_string = String::from("");
    for col_num in 0..columns {
        let bit_int = most_common(nums, col_num, gamma);
        let bit_char = match bit_int {
            1 => '1',
            _ => '0',
        };
        bit_string.push(bit_char);
    }
    return u64::from_str_radix(bit_string.as_str(), 2).unwrap();
}

fn generate_winning_num(boards: &[Vec<u32>], gamma: bool) -> u64 {
    let columns = boards[0].len();
    // let mut bit_string = String::from("");
    let mut nums = boards.to_vec();
    for col_num in 0..columns {
        let common = most_common(&nums, col_num, gamma);
        nums.retain(|row| row[col_num] == common);
        if nums.len() == 1 {
            break;
        }
    }
    let bit_chars: Vec<&str> = nums[0]
        .iter()
        .map(|num| match num {
            1 => "1",
            0 => "0",
            _ => panic!("unknown {}", num),
        })
        .collect::<Vec<&str>>();
    let bit_string = bit_chars.join("");
    return u64::from_str_radix(bit_string.as_str(), 2).unwrap();
}

pub fn part1() -> u64 {
    let binaries = get_binaries("data/binaries.txt");
    let new_num = generate_new_num(&binaries, true);
    let new_num_reverse = generate_new_num(&binaries, false);
    let consumption = new_num * new_num_reverse;
    assert_eq!(consumption, 2003336);
    consumption
}

pub fn part2() -> u64 {
    let binaries = get_binaries("data/binaries.txt");
    let new_num = generate_winning_num(&binaries, true);
    let new_num_reverse = generate_winning_num(&binaries, false);
    let life_support_rating = new_num * new_num_reverse;
    assert_eq!(life_support_rating, 1877139);
    life_support_rating
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_new_num() {
        let binaries = super::get_binaries("../fixtures/binaries.txt");
        let new_num = super::generate_new_num(&binaries, true);
        let new_num_reverse = super::generate_new_num(&binaries, false);

        assert_eq!(new_num * new_num_reverse, 198);
    }

    #[test]
    fn test_generate_winning_num() {
        let binaries = super::get_binaries("../fixtures/binaries.txt");
        let new_num = super::generate_winning_num(&binaries, true);
        let new_num_reverse = super::generate_winning_num(&binaries, false);
        assert_eq!(new_num * new_num_reverse, 230);
    }
}
