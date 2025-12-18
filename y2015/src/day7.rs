use std::collections::HashMap;

use itertools::Itertools;

struct Expression<'a> {
    a: &'a str,
    b: &'a str,
    cmd: Command,
}

impl<'a> Expression<'a> {
    fn new(a: &'a str, b: &'a str, cmd: Command) -> Self {
        Self { a, b, cmd }
    }

    fn new2(a: &'a str, cmd: Command) -> Self {
        Self { a, b: "", cmd }
    }
}

enum Command {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Assign,
}
#[allow(dead_code)]
fn part_1(input: &str) -> u16 {
    let wires = parse_input(input);
    let mut cache = HashMap::with_capacity(wires.len());
    eval("a", &wires, &mut cache)
}

fn parse_input(input: &'_ str) -> HashMap<&'_ str, Expression<'_>> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        match line.split_whitespace().collect_vec().as_slice() {
            [w1, "AND", w2, "->", w3] => {
                wires.insert(*w3, Expression::new(w1, w2, Command::And));
            }
            [w1, "OR", w2, "->", w3] => {
                wires.insert(*w3, Expression::new(w1, w2, Command::Or));
            }
            [w1, "LSHIFT", w2, "->", w3] => {
                wires.insert(*w3, Expression::new(w1, w2, Command::Lshift));
            }
            [w1, "RSHIFT", w2, "->", w3] => {
                wires.insert(*w3, Expression::new(w1, w2, Command::Rshift));
            }
            ["NOT", w1, "->", w2] => {
                wires.insert(w2, Expression::new2(w1, Command::Not));
            }
            [w1, "->", w2] => {
                wires.insert(w2, Expression::new2(w1, Command::Assign));
            }
            _ => panic!("invalid input"),
        };
    }
    wires
}

fn eval<'a>(
    value: &'a str,
    wires: &'a HashMap<&'a str, Expression>,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if value.is_empty() {
        return 0;
    }
    if let Ok(num) = value.parse() {
        return num;
    }
    if cache.contains_key(value) {
        return *cache.get(value).unwrap();
    }

    let expr = wires.get(value).unwrap();
    let a = eval(expr.a, wires, cache);
    cache.insert(expr.a, a);
    let b = eval(expr.b, wires, cache);
    cache.insert(expr.b, b);
    let eval = match expr.cmd {
        Command::And => a & b,
        Command::Or => a | b,
        Command::Lshift => a << b,
        Command::Rshift => a >> b,
        Command::Not => !a,
        Command::Assign => a,
    };
    cache.insert(value, eval);
    eval
}

#[allow(dead_code)]
fn part_2(input: &str) -> u16 {
    let mut wires = parse_input(input);
    let mut cache = HashMap::with_capacity(wires.len());
    let a = format!("{}", eval("a", &wires, &mut cache));
    wires.insert(
        "b",
        Expression {
            a: &a,
            b: "",
            cmd: Command::Assign,
        },
    );

    let mut cache = HashMap::with_capacity(wires.len());
    eval("a", &wires, &mut cache)
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
        assert_eq!(part_1(&input), 16076);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(7);
        assert_eq!(part_2(&input), 2797);
    }
}
