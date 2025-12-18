use std::collections::HashMap;

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> u16 {
    let mut wires: HashMap<&str, u16> = HashMap::new();
    for line in input.lines() {
        match line.split_whitespace().collect_vec().as_slice() {
            [w1, "AND", w2, "->", w3] => {
                let a = wires.get(w1).unwrap_or(&0);
                let b = wires.get(w2).unwrap_or(&0);
                wires.insert(w3, a & b);
            }
            [w1, "OR", w2, "->", w3] => {
                let a = wires.get(w1).unwrap_or(&0);
                let b = wires.get(w2).unwrap_or(&0);
                wires.insert(w3, a | b);
            }
            [w1, "LSHIFT", v, "->", w3] => {
                let a = wires.get(w1).unwrap_or(&0);
                let b = v.parse::<usize>().unwrap();
                wires.insert(w3, a << b);
            }
            [w1, "RSHIFT", v, "->", w3] => {
                let a = wires.get(w1).unwrap_or(&0);
                let b = v.parse::<usize>().unwrap();
                wires.insert(w3, a >> b);
            }
            ["NOT", w1, "->", w2] => {
                let a = wires.get(w1).unwrap_or(&0);
                wires.insert(w2, !a);
            }
            [w1, "->", w2] => {
                let a = w1
                    .parse()
                    .unwrap_or_else(|_| wires.get(w1).copied().unwrap_or(0));
                wires.insert(w2, a);
            }
            _ => panic!("invalid input"),
        };
    }
    dbg!(&wires);
    *wires.get("a").unwrap()
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
        let input = r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> a
NOT y -> i";
        assert_eq!(part_1(input), 65412);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(7);
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(7);
        assert_eq!(part_2(&input), 0);
    }
}
