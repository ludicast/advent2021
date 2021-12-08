use std::collections::HashSet;

use file_reader::{get_bingos, BingoGame};

fn get_last_bingo_winner(bingos: BingoGame) -> i64 {
    let len = bingos.boards.len();

    let ball_sack: &mut HashSet<i64> = &mut HashSet::new();
    let old_wins: &mut HashSet<usize> = &mut HashSet::new();

    for ball in bingos.balls {
        ball_sack.insert(ball);
        for i in 0..len {
            if old_wins.contains(&i) {
                continue;
            }
            let board = &bingos.boards[i];
            if board.has_bingo(ball_sack) {
                if old_wins.len() == len - 1 {
                    let (v1, v2) = board.matched(ball_sack);
                    return v2.iter().sum::<i64>() * ball;
                }
                old_wins.insert(i);
            }
        }
    }
    return -1;
}

pub fn day4_part2() {
    let bingos = get_bingos("data/bingos.txt");
    get_last_bingo_winner(bingos);
}

fn get_bingo_winner(bingos: BingoGame) -> i64 {
    let ball_sack = &mut HashSet::new();
    for ball in bingos.balls {
        ball_sack.insert(ball);
        for board in &bingos.boards {
            if board.has_bingo(ball_sack) {
                let (v1, v2) = board.matched(ball_sack);
                return v2.iter().sum::<i64>() * ball;
            }
        }
    }
    return -1;
}

pub fn day4_part1() {
    let bingos = get_bingos("data/bingos.txt");
    get_bingo_winner(bingos);
}

pub fn day4() {
    day4_part1();
    day4_part2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_bingo_winner() {
        let bingos = super::get_bingos("../fixtures/bingos.txt");
        let winner = super::get_bingo_winner(bingos);

        assert_eq!(winner, 4512);
    }

    #[test]
    fn test_get_last_bingo_winner() {
        let bingos = super::get_bingos("../fixtures/bingos.txt");
        let winner = super::get_last_bingo_winner(bingos);

        assert_eq!(winner, 1924);
    }
}
