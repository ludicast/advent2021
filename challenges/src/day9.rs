use file_reader::get_map;

fn is_lowpoint(map: &Vec<Vec<u8>>, row_idx: usize, col_idx: usize) -> Option<u8> {
    let value = map[row_idx][col_idx];
    let last_row = map.len() - 1;
    let last_col = map[0].len() - 1;

    if row_idx > 0 && value >= map[row_idx - 1][col_idx] {
        return None;
    };
    if row_idx < last_row && value >= map[row_idx + 1][col_idx] {
        return None;
    };
    if col_idx > 0 && value >= map[row_idx][col_idx - 1] {
        return None;
    };
    if col_idx < last_col && value >= map[row_idx][col_idx + 1] {
        return None;
    };
    Some(value)
}

fn collect_lowpoints(map: &Vec<Vec<u8>>) -> Vec<u8> {
    let row_count = map.len();
    let col_count = map[0].len();
    let mut lowpoints = vec![];
    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            if let Some(value) = is_lowpoint(map, row_idx, col_idx) {
                lowpoints.push(value);
            }
        }
    }
    lowpoints
}

fn sum_risk_levels(map: &Vec<Vec<u8>>) -> u64 {
    collect_lowpoints(map)
        .iter()
        .map(|val| *val as u64 + 1)
        .sum()
}

pub fn day9() {
    let map = get_map("data/map.txt");
    println!("sum risk: {}", sum_risk_levels(&map));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_collect_lowpoints() {
        let map = super::get_map("../fixtures/map.txt");
        let lowpoints = super::collect_lowpoints(&map);
        assert_eq!(lowpoints, vec![1, 0, 5, 5]);
    }

    #[test]
    fn test_sum_risk_levels() {
        let map = super::get_map("../fixtures/map.txt");
        let risk_levels = super::sum_risk_levels(&map);
        assert_eq!(risk_levels, 15);
    }
}
