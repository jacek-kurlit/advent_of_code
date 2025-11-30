#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    input.len()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"";
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(1);
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(1);
        assert_eq!(part_2(&input), 0);
    }
}
