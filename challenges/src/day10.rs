use file_reader::get_braces;

fn score_completion(chars: &[char]) -> i64 {
    chars
        .iter()
        .rev()
        .map(|val| match val {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("stack error: {}", val),
        })
        .reduce(|a, b| a * 5 + b)
        .unwrap()
}

fn score_of_line(chars: &[char]) -> Result<i64, i64> {
    let mut stack = vec![];
    for char in chars.iter() {
        let last = stack.last();
        match (last, char) {
            (Some('('), ')') => {
                stack.pop();
            }
            (Some('['), ']') => {
                stack.pop();
            }
            (Some('<'), '>') => {
                stack.pop();
            }
            (Some('{'), '}') => {
                stack.pop();
            }
            (_, ')') => return Err(3),
            (_, ']') => return Err(57),
            (_, '>') => return Err(25137),
            (_, '}') => return Err(1197),
            _ => {
                stack.push(*char);
            }
        }
    }
    Ok(score_completion(&stack))
}

fn calculate_corruption_score(nums: &[Vec<char>]) -> i64 {
    nums.iter()
        .map(|chars| match score_of_line(chars).err() {
            Some(val) => val,
            _ => 0,
        })
        .sum()
}

fn calculate_incompleteness_score(nums: &[Vec<char>]) -> i64 {
    let mut scores = nums
        .iter()
        .filter_map(|chars| score_of_line(chars).ok())
        .collect::<Vec<i64>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub fn part1() -> i64 {
    let braces = get_braces("data/braces.txt");
    let score = calculate_corruption_score(&braces);
    assert!(score == 358737);
    score
}

pub fn part2() -> i64 {
    let braces = get_braces("data/braces.txt");
    let score = calculate_incompleteness_score(&braces);
    assert_eq!(score, 4329504793);
    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_corruption_score() {
        let braces = super::get_braces("../fixtures/braces.txt");
        let score = super::calculate_corruption_score(&braces);
        assert_eq!(score, 26397);
    }
    #[test]
    fn test_calculate_incompleteness_score() {
        let braces = super::get_braces("../fixtures/braces.txt");
        let score = super::calculate_incompleteness_score(&braces);
        assert_eq!(score, 288957);
    }
}
