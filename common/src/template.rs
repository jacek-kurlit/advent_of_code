#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    input.len() as u32
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"";
        assert_eq!(part_1(input), 142);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/d1_p1.txt");
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/d1_p2.txt");
        assert_eq!(part_2(input), 0);
    }
}
