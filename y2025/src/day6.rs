use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    let Sheet { numbers, operators } = parse_input(input);
    let mut sum = 0;
    let columns = numbers[0].len();

    (0..columns).for_each(|column| {
        let value = calculate(&numbers, column, operators[column]);
        sum += value;
    });

    sum
}

fn calculate(numbers: &[Vec<u64>], column: usize, operator: char) -> u64 {
    let values: Vec<u64> = numbers.iter().map(|row| row[column]).collect_vec();
    values
        .into_iter()
        .reduce(|acc, v| match operator {
            '*' => acc * v,
            '+' => acc + v,
            _ => panic!("Nope"),
        })
        .unwrap_or(0)
}

#[derive(Debug)]
struct Sheet {
    numbers: Vec<Vec<u64>>,
    operators: Vec<char>,
}

fn parse_input(input: &str) -> Sheet {
    let lines: Vec<&str> = input.trim().lines().collect();
    let numbers = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split(char::is_whitespace)
                .filter(|s| !s.is_empty())
                .map(|number| number.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect();
    let operators = lines[lines.len() - 1].replace(" ", "").chars().collect();
    Sheet { numbers, operators }
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let numbers: Vec<Vec<u64>> = as_numbers(&lines[0..lines.len() - 1]);
    let operators = lines
        .last()
        .unwrap()
        .iter()
        .filter(|c| **c != ' ')
        .collect_vec();
    let mut sum = 0;
    for (i, n) in numbers.into_iter().enumerate() {
        sum += n
            .into_iter()
            .reduce(|acc, v| match operators[i] {
                '*' => acc * v,
                '+' => acc + v,
                _ => panic!("Nope"),
            })
            .unwrap();
    }
    sum
}

fn as_numbers(input: &[Vec<char>]) -> Vec<Vec<u64>> {
    let mut numbers = vec![];
    let mut column_numbers = vec![];
    for i in 0..input[0].len() {
        let n = read_column_number(input, i);
        if let Some(n) = n {
            column_numbers.push(n);
        } else {
            numbers.push(column_numbers);
            column_numbers = vec![];
        }
    }
    numbers.push(column_numbers);
    numbers
}

fn read_column_number(input: &[Vec<char>], column: usize) -> Option<u64> {
    let mut number: u64 = 0;
    for row in input {
        let c = row[column];
        if c.is_ascii_digit() {
            number = number * 10 + c.to_digit(10).unwrap() as u64;
        }
    }
    if number > 0 { Some(number) } else { None }
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part_1(input), 4277556);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(6);
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part_2(input), 3263827);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(6);
        assert_eq!(part_2(&input), 0);
    }
}
