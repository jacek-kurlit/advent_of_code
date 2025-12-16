use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|(w, l, h)| {
            let a = w * l;
            let b = w * h;
            let c = h * l;
            2 * a + 2 * b + 2 * c + a.min(b).min(c)
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|l| {
            let mut values = l.split("x");
            let as_values = |v: Option<&str>| v.unwrap().parse::<u64>().unwrap();
            (
                as_values(values.next()),
                as_values(values.next()),
                as_values(values.next()),
            )
        })
        .collect_vec()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|(w, l, h)| {
            let m = vec![w + l, l + h, h + w].into_iter().min().unwrap();
            2 * m + w * l * h
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"2x3x4
1x1x10";
        assert_eq!(part_1(input), 101);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(2);
        assert_eq!(part_1(&input), 1588178);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"2x3x4
1x1x10";
        assert_eq!(part_2(input), 48);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(2);
        assert_eq!(part_2(&input), 3783758);
    }
}
