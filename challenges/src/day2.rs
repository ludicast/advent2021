use file_reader::get_directions;

fn location_position(moves: &Vec<(String, i64)>) -> (i64, i64) {
    moves.iter().fold(
        (0, 0),
        |(x_coord, y_coord), (direction, magnitude)| match direction.as_str() {
            "forward" => (x_coord + magnitude, y_coord),
            "down" => (x_coord, y_coord + magnitude),
            "up" => (x_coord, y_coord - magnitude),
            _ => (x_coord, y_coord),
        },
    )
}

fn aimed_location_position(moves: &Vec<(String, i64)>) -> (i64, i64, i64) {
    moves.iter().fold(
        (0, 0, 0),
        |(x_coord, y_coord, aim), (direction, magnitude)| match direction.as_str() {
            "forward" => (x_coord + magnitude, y_coord + aim * magnitude, aim),
            "down" => (x_coord, y_coord, aim + magnitude),
            "up" => (x_coord, y_coord, aim - magnitude),
            _ => (x_coord, y_coord, aim),
        },
    )
}

pub fn day2() {
    let directions = get_directions("data/directions.txt");
    let (x_coord, y_coord) = location_position(&directions);
    let (x_coord, y_coord, aim) = aimed_location_position(&directions);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_location_position() {
        let directions = super::get_directions("../fixtures/directions.txt");
        let (x_coord, y_coord) = super::location_position(&directions);
        assert_eq!(x_coord * y_coord, 150);
    }

    #[test]
    fn test_aimed_location_position() {
        let directions = super::get_directions("../fixtures/directions.txt");
        let (x_coord, y_coord, _aim) = super::aimed_location_position(&directions);
        assert_eq!(x_coord * y_coord, 900);
    }
}
