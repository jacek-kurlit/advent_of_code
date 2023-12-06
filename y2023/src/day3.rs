use std::ops::RangeInclusive;

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    let lines = parse_input(input);
    (0..lines.len())
        .flat_map(|i| numbers_with_adjecent_symbols(i, &lines))
        .map(|n| n.value.parse::<u32>().expect("Invalid number"))
        .sum()
}

fn numbers_with_adjecent_symbols(line_index: usize, lines: &[ParsedLine]) -> Vec<&Number> {
    let current_line = lines.get(line_index).unwrap();
    let mut result = vec![];
    for number in &current_line.numbers {
        if has_symbol_in_range(
            lines.get(line_index.saturating_sub(1)),
            &number.adjacent_range,
        ) || has_symbol_in_range(Some(current_line), &number.adjacent_range)
            || has_symbol_in_range(lines.get(line_index + 1), &number.adjacent_range)
        {
            result.push(number);
        }
    }
    result
}

fn has_symbol_in_range(line: Option<&ParsedLine>, adjacent_range: &RangeInclusive<usize>) -> bool {
    if line.is_none() {
        return false;
    }
    let line = line.unwrap();
    line.symbols.iter().any(|s| adjacent_range.contains(&s.pos))
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    let lines = parse_input(input);
    (0..lines.len())
        .flat_map(|i| gears_with_adjecent_numbers(i, &lines))
        .sum()
}
fn gears_with_adjecent_numbers(line_index: usize, lines: &[ParsedLine]) -> Vec<u32> {
    let current_line = lines.get(line_index).unwrap();
    let mut result = vec![];
    for symbol in &current_line.symbols {
        if symbol.value != '*' {
            continue;
        }

        let up = all_adjacent_numbers(lines.get(line_index.saturating_sub(1)), &symbol.pos);
        let current = all_adjacent_numbers(Some(current_line), &symbol.pos);
        let below = all_adjacent_numbers(lines.get(line_index + 1), &symbol.pos);
        let all_adjacent_numbers = up
            .iter()
            .chain(current.iter())
            .chain(below.iter())
            .collect::<Vec<&&Number>>();
        if all_adjacent_numbers.len() == 2 {
            let product = all_adjacent_numbers[0]
                .value
                .parse::<u32>()
                .expect("Invalid number")
                * all_adjacent_numbers[1]
                    .value
                    .parse::<u32>()
                    .expect("Invalid number");
            result.push(product);
        }
    }
    result
}

fn all_adjacent_numbers<'a>(line: Option<&'a ParsedLine>, symbol_pos: &usize) -> Vec<&'a Number> {
    if line.is_none() {
        return vec![];
    }
    let line = line.unwrap();
    line.numbers
        .iter()
        .filter(|n| n.adjacent_range.contains(symbol_pos))
        .collect()
}

fn parse_input(input: &str) -> Vec<ParsedLine> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> ParsedLine {
    let mut symbols = vec![];
    let mut numbers = vec![];
    let chars = line.chars().collect::<Vec<char>>();
    let mut index = 0;
    while index < chars.len() {
        let offset = match &chars[index..] {
            [c, rest @ ..] if c.is_ascii_digit() => {
                let rest_digits: String = rest.iter().take_while(|x| x.is_ascii_digit()).collect();
                numbers.push(Number {
                    value: format!("{}{}", c, rest_digits),
                    adjacent_range: index.saturating_sub(1)..=(index + rest_digits.len() + 1),
                });

                rest_digits.len() + 1
            }
            ['.', ..] => 1,
            [c, ..] => {
                symbols.push(Symbol {
                    pos: index,
                    value: *c,
                });
                1
            }
            _ => panic!("Invalid state"),
        };
        index += offset;
    }

    ParsedLine { symbols, numbers }
}
struct ParsedLine {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

struct Symbol {
    pos: usize,
    value: char,
}

#[derive(Clone)]
struct Number {
    value: String,
    adjacent_range: RangeInclusive<usize>,
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_1(input), 4361);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day3.txt");
        assert_eq!(part_1(input), 537832);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_2(input), 467835);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day3.txt");
        assert_eq!(part_2(input), 81939900);
    }
}
