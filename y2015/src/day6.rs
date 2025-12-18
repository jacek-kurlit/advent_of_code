use common::coordinates::Vec2;
use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let mut matrix = vec![vec![false; 1000]; 1000];
    let input = parse_input(input);
    for (command, from, to) in input {
        (from.x..=to.x).for_each(|row| {
            (from.y..=to.y).for_each(|column| {
                let new_value = match command {
                    Command::TurnOn => true,
                    Command::TurnOff => false,
                    Command::Toggle => !matrix[row][column],
                };
                matrix[row][column] = new_value;
            });
        });
    }
    matrix
        .into_iter()
        .map(|row| row.iter().filter(|&&is_on| is_on).count())
        .sum()
}

fn parse_input(input: &str) -> Vec<(Command, Vec2<usize>, Vec2<usize>)> {
    let parse_point = |s: &str| {
        let (x, y) = s.split_once(",").unwrap();
        Vec2::new(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    };
    input
        .lines()
        .map(|l| match &l.split_whitespace().collect_vec().as_slice() {
            ["turn", "on", p1, "through", p2] => {
                (Command::TurnOn, parse_point(p1), parse_point(p2))
            }
            ["turn", "off", p1, "through", p2] => {
                (Command::TurnOff, parse_point(p1), parse_point(p2))
            }
            ["toggle", p1, "through", p2] => (Command::Toggle, parse_point(p1), parse_point(p2)),
            _ => panic!("invalid input"),
        })
        .collect()
}

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let mut matrix = vec![vec![0; 1000]; 1000];
    let input = parse_input(input);
    for (command, from, to) in input {
        (from.x..=to.x).for_each(|row| {
            (from.y..=to.y).for_each(|column| {
                let new_value = match command {
                    Command::TurnOn => 1,
                    Command::TurnOff => -1,
                    Command::Toggle => 2,
                };
                matrix[row][column] = (matrix[row][column] + new_value).max(0);
            });
        });
    }
    matrix
        .into_iter()
        .map(|row| row.iter().filter(|&&value| value > 0).sum::<i32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        assert_eq!(part_1(input), 998996);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(6);
        assert_eq!(part_1(&input), 569999);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"turn on 0,0 through 0,0
toggle 0,0 through 999,999";
        assert_eq!(part_2(input), 2_000_001);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(6);
        assert_eq!(part_2(&input), 17836115);
    }
}
