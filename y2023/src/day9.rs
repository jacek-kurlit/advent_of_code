#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let parsed_data = parse_lines(input);
    parsed_data
        .into_iter()
        .map(interpolate)
        .map(predict_next_value)
        .sum()
}

fn predict_next_value(row_interpolations: Vec<Vec<i32>>) -> i32 {
    row_interpolations
        .iter()
        .map(|v| v.last().expect("Last value"))
        .sum::<i32>()
}

fn interpolate(data: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current_interpolation = data;
    while current_interpolation.iter().any(|&x| x != 0) {
        result.push(current_interpolation);
        current_interpolation = result
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
    }
    result
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let parsed_data = parse_lines(input);
    parsed_data
        .into_iter()
        .map(interpolate)
        .map(predict_previous_value)
        .sum()
}

fn predict_previous_value(interpolations: Vec<Vec<i32>>) -> i32 {
    interpolations
        .iter()
        .map(|v| v.first().expect("No first value"))
        .rev()
        .fold(0, |acc, n| n - acc)
}

fn parse_lines(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.trim().split(' ').map(|s| s.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_1(input), 114);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day9.txt");
        assert_eq!(part_1(input), 1725987467);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_2(input), 2);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day9.txt");
        assert_eq!(part_2(input), 971);
    }
}
