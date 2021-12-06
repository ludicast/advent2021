use util::read_lines;

use std::collections::HashSet;

#[derive(PartialEq)]
pub struct Board {
    pub values: Vec<Vec<i64>>,
    pub value_set: HashSet<i64>,
}

impl Board {
    pub fn new(values: Vec<Vec<i64>>) -> Self {
        let mut value_set = HashSet::new();
        for row in &values {
            for cell in row {
                value_set.insert(*cell);
            }
        }
        Board { values, value_set }
    }

    fn columns(&self) -> Vec<Vec<i64>> {
        let column_count = self.values.len();
        (0..column_count)
            .map(|col_num| {
                self.values
                    .iter()
                    .map(|row| row[col_num])
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>()
    }

    pub fn has_bingo(&self, nums: &mut HashSet<i64>) -> bool {
        for row in &self.values {
            if row.iter().all(|item| nums.contains(item)) {
                return true;
            }
        }
        for col in self.columns() {
            if col.iter().all(|item| nums.contains(item)) {
                return true;
            }
        }
        false
    }

    pub fn matched(&self, nums: &mut HashSet<i64>) -> (Vec<i64>, Vec<i64>) {
        let values = self.values.clone();

        let matched_cells = values
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|cell| nums.contains(cell))
                    .map(|cell| *cell)
            })
            .collect::<Vec<i64>>();
        let non_matched_cells = values
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|cell| !nums.contains(cell))
                    .map(|cell| *cell)
            })
            .collect::<Vec<i64>>();
        (matched_cells, non_matched_cells)
    }
}

pub struct BingoGame {
    pub balls: Vec<i64>,
    pub boards: Vec<Board>,
}

pub fn get_bingos(filename: &str) -> BingoGame {
    let lines = read_lines(filename);
    let balls = lines[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let games_count = (lines.len() - 1) / 6;

    let boards: Vec<Board> = (0..games_count)
        .map(|game_num| {
            let start = 2 + game_num * 6;
            let values = (start..start + 5)
                .map(|row_num| {
                    lines[row_num]
                        .split(" ")
                        // some spots are double-spaced, but regex is a 3rd party util
                        .filter(|piece| *piece != "")
                        .map(|num_string| i64::from_str_radix(num_string, 10).unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();
            Board::new(values)
        })
        .collect::<Vec<Board>>();

    BingoGame { balls, boards }
}
