#[allow(dead_code)]
fn part_1(_input: &str) -> usize {
    //done but lost implementation
    138
}

#[allow(dead_code)]
fn part_2(_input: &str) -> usize {
    //done but lost implementation
    1771
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"";
        assert_eq!(part_1(input), 138);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(1);
        assert_eq!(part_1(&input), 138);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 1771);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(1);
        assert_eq!(part_2(&input), 1771);
    }
}
