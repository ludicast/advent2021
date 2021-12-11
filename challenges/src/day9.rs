use std::collections::HashMap;

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

fn follow_pool(
    map: &Vec<Vec<u8>>,
    pool_map: &mut HashMap<(i32, i32), u32>,
    pool_num: u32,
    row: i32,
    col: i32,
) {
    let row_count = map.len() as i32;
    let col_count = map[0].len() as i32;
    if row < 0 || row >= row_count || col < 0 || col >= col_count {
        return;
    }
    if map[row as usize][col as usize] == 9 {
        return;
    }
    if pool_map.contains_key(&(row, col)) {
        return;
    }
    pool_map.insert((row, col), pool_num);

    follow_pool(map, pool_map, pool_num, row - 1, col);
    follow_pool(map, pool_map, pool_num, row + 1, col);
    follow_pool(map, pool_map, pool_num, row, col - 1);
    follow_pool(map, pool_map, pool_num, row, col + 1);
}

fn find_pools(map: &Vec<Vec<u8>>) -> HashMap<(i32, i32), u32> {
    let mut hash_map = HashMap::new();
    let mut pool_num = 0;

    let row_count = map.len();
    let col_count = map[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            if !hash_map.contains_key(&(row as i32, col as i32)) && map[row][col] != 9 {
                follow_pool(map, &mut hash_map, pool_num, row as i32, col as i32);
                pool_num += 1;
            }
        }
    }
    hash_map
}
fn get_basin_product(map: &Vec<Vec<u8>>) -> i32 {
    let pools = find_pools(&map);
    let mut pool_sizes = HashMap::new();
    pools.values().for_each(|value| {
        let pool_size = match pool_sizes.get(value) {
            Some(v) => *v,
            None => 0,
        };
        pool_sizes.insert(value, pool_size + 1);
    });

    let mut values = pool_sizes.values().collect::<Vec<&i32>>();
    values.sort();
    values.reverse();

    return values[0] * values[1] * values[2];
}

pub fn day9() {
    let map = get_map("data/map.txt");
    println!("sum risk: {}", sum_risk_levels(&map));
    println!("basin_product: {}", get_basin_product(&map));
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

    #[test]
    fn test_find_pools() {
        let map = super::get_map("../fixtures/map.txt");
        let basin_product = super::get_basin_product(&map);
        assert_eq!(basin_product, 1134);
    }
}
