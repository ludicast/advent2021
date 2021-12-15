use file_reader::get_octopuses;

fn advance_neighbors(octopuses: &mut [[u32; 10]; 10], row: usize, col: usize) {
    if row > 0 {
        if col > 0 {
            octopuses[row - 1][col - 1] += 1;
        }
        octopuses[row - 1][col] += 1;
        if col < 9 {
            octopuses[row - 1][col + 1] += 1;
        }
    }
    if col > 0 {
        octopuses[row][col - 1] += 1;
    }
    if col < 9 {
        octopuses[row][col + 1] += 1;
    }
    if row < 9 {
        if col > 0 {
            octopuses[row + 1][col - 1] += 1;
        }
        octopuses[row + 1][col] += 1;
        if col < 9 {
            octopuses[row + 1][col + 1] += 1;
        }
    }
}

fn trigger_flashes(octopuses: &mut [[u32; 10]; 10]) -> u32 {
    let mut flashes = 0;
    let mut has_new_flashes = true;
    let mut recent_flashes: [[bool; 10]; 10] = Default::default();

    while has_new_flashes {
        has_new_flashes = false;

        for row in 0..10 {
            for col in 0..10 {
                if octopuses[row][col] > 9 && !recent_flashes[row][col] {
                    recent_flashes[row][col] = true;
                    flashes += 1;
                    has_new_flashes = true;
                    advance_neighbors(octopuses, row, col);
                }
            }
        }
    }

    for row in 0..10 {
        for col in 0..10 {
            if recent_flashes[row][col] {
                octopuses[row][col] = 0;
            }
        }
    }
    return flashes;
}

fn bump_all(octopuses: &mut [[u32; 10]; 10]) {
    for row in 0..10 {
        for col in 0..10 {
            octopuses[row][col] += 1;
        }
    }
}

fn take_step(octopuses: &mut [[u32; 10]; 10]) -> u32 {
    bump_all(octopuses);
    let flashes = trigger_flashes(octopuses);
    flashes
}

fn take_steps(octopuses: &mut [[u32; 10]; 10], count: usize) -> u32 {
    (0..count).fold(0, |acc, _| {
        let flashes = take_step(octopuses);
        acc + flashes
    })
}

fn all_zeros(octopuses: &[[u32; 10]; 10]) -> bool {
    let iter = octopuses.iter();
    iter.map(|row: &[u32; 10]| row.iter().sum::<u32>())
        .sum::<u32>()
        == 0
}

fn get_first_flash(octopuses: &mut [[u32; 10]; 10]) -> u32 {
    let mut first_flash = 0;
    while !all_zeros(octopuses) {
        take_step(octopuses);
        first_flash += 1;
    }
    first_flash
}

pub fn part1() -> u32 {
    let mut octopuses = get_octopuses("data/octopuses.txt");
    let score = take_steps(&mut octopuses, 100);
    assert_eq!(score, 1688);
    score
}

pub fn part2() -> u32 {
    let mut octopuses = get_octopuses("data/octopuses.txt");
    let first_flash = get_first_flash(&mut octopuses);
    assert_eq!(first_flash, 403);
    first_flash
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_take_step() {
        let mut octopuses = super::get_octopuses("../fixtures/octopuses.txt");
        let mut flashes = super::take_step(&mut octopuses);
        assert_eq!(flashes, 0);
        flashes = super::take_step(&mut octopuses);
        assert_eq!(flashes, 35);
        flashes = super::take_step(&mut octopuses);
        assert_eq!(flashes, 45);
    }

    #[test]
    fn test_take_steps() {
        let mut octopuses = super::get_octopuses("../fixtures/octopuses.txt");
        let mut flashes = super::take_steps(&mut octopuses, 10);
        assert_eq!(flashes, 204);
    }

    #[test]
    fn test_take_many_steps() {
        let mut octopuses = super::get_octopuses("../fixtures/octopuses.txt");
        let mut flashes = super::take_steps(&mut octopuses, 100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_get_first_flash() {
        let mut octopuses = super::get_octopuses("../fixtures/octopuses.txt");
        let first_flash = super::get_first_flash(&mut octopuses);
        assert_eq!(first_flash, 195);
    }
}
