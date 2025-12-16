use std::collections::HashMap;

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let dirs = parse_input(input);
    let mut homes = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    homes.insert((x, y), 1);
    for dir in dirs {
        match dir {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!("invalid input"),
        }
        let count = homes.entry((x, y)).or_insert(0);
        *count += 1;
    }

    homes.values().count()
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect_vec()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let dirs = parse_input(input);
    let mut homes = HashMap::new();
    let mut santas = [(0, 0), (0, 0)];
    let mut santa_nr = 0;
    homes.insert((0, 0), 2);
    for dir in dirs {
        let santa = santas.get_mut(santa_nr).unwrap();
        let (x, y) = santa;
        match dir {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => panic!("invalid input"),
        }
        santa_nr = (santa_nr + 1) % 2;
        let count = homes.entry((*x, *y)).or_insert(0);
        *count += 1;
    }

    homes.values().count()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"^>v<";
        assert_eq!(part_1(input), 4);
        let input = r"^v^v^v^v^v";
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(3);
        assert_eq!(part_1(&input), 2572);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"^v";
        assert_eq!(part_2(input), 3);
        let input = r"^>v<";
        assert_eq!(part_2(input), 3);
        let input = r"^v^v^v^v^v";
        assert_eq!(part_2(input), 11);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(3);
        assert_eq!(part_2(&input), 2631);
    }
}
