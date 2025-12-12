use std::collections::HashSet;

use common::algorithms;
use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .flat_map(|(from, to)| generate_invalid_ids(from, to))
        .sum()
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let mut values = range.trim().split("-");
            (values.next().unwrap(), values.next().unwrap())
        })
        .collect()
}

fn generate_invalid_ids(range_start: &str, range_end: &str) -> Vec<u64> {
    let mut result = vec![];
    //TODO: this can be less than range_start still
    let (mut first_half, mut base) = find_range_start(range_start);
    let mut current = first_half * base + first_half;
    let end = range_end.parse::<u64>().unwrap();
    while current <= end {
        result.push(current);
        first_half += 1;
        if first_half >= base {
            base *= 10;
        }
        current = first_half * base + first_half;
    }
    result
}

fn find_range_start(range_start: &str) -> (u64, u64) {
    // "1" -> (1, 10)
    // "12" -> (2, 10) we need to skip 1 since it is less than 2 already
    // "123" -> (10, 100)
    // "1234" -> (13, 100) since we skip 12 because it is less than 34
    // "12345" -> (100, 1000)
    // "123456" -> (100, 1000)

    let pivot = range_start.len() / 2;
    // for even we can use input start
    if range_start.len().is_multiple_of(2) {
        let mut first_half: u64 = range_start[0..pivot].parse().unwrap();
        let second_half: u64 = range_start[pivot..].parse().unwrap();
        //adapt range accordingly
        if first_half < second_half {
            first_half += 1;
        }
        let base = 10u64.pow(pivot as u32);
        return (first_half, base);
    }
    //for odd we can calculate next valid start
    let first_half = 10u64.pow(pivot as u32);
    (first_half, first_half * 10)
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|(from, to)| generate_invalid_ids_v2(from, to))
        .sum()
}

fn generate_invalid_ids_v2(range_start: &str, range_end: &str) -> u64 {
    let mut result = 0;
    let start_value = range_start.parse().unwrap();
    let end_value = range_end.parse().unwrap();
    for number_of_digits in range_start.len() as u64..=range_end.len() as u64 {
        let divisors = algorithms::divisors::divisors(number_of_digits);
        if divisors.len() <= 1 {
            continue;
        }
        let values = if divisors.len() == 2 {
            generate_numbers_of_length(1, number_of_digits, start_value, end_value)
        } else {
            let without_last = divisors.len() - 2;
            let divisors = divisors
                .into_iter()
                .skip(1)
                .take(without_last)
                .collect_vec();
            generate_ids_for(divisors, number_of_digits, start_value, end_value)
        };
        dbg!(start_value, end_value, &values);
        result += values.into_iter().sum::<u64>();
    }

    result
}

fn generate_numbers_of_length(
    sub_length: u64,
    target_length: u64,
    start_value: u64,
    end_value: u64,
) -> Vec<u64> {
    let times = (target_length / sub_length) as usize;
    let start = 10u64.pow(sub_length as u32 - 1);
    (0..)
        .map(|ind| {
            let current = start + ind;
            format!("{current}").repeat(times).parse().unwrap()
        })
        .skip_while(|value| *value < start_value)
        .take_while(|value| *value <= end_value)
        .collect_vec()
}

fn generate_ids_for(divisors: Vec<u64>, number_of_digits: u64, start: u64, end: u64) -> Vec<u64> {
    let mut result = HashSet::new();
    for divisor in divisors {
        generate_numbers_of_length(divisor, number_of_digits, start, end)
            .into_iter()
            .for_each(|v| {
                result.insert(v);
            });
    }
    result.into_iter().collect_vec()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn part_1_simple_range() {
        let input = r"10-99";
        assert_eq!(part_1(input), 495);
    }

    #[test]
    fn part_1_odd_range_size_start() {
        let input = r"1-99";
        assert_eq!(part_1(input), 495);
    }

    #[test]
    fn part_1_odd_range_size_end() {
        let input = r"1-999";
        assert_eq!(part_1(input), 495);
    }

    #[test]
    fn part_1_simple_range_starts_from_12() {
        // we should not count 11
        let input = r"12-99";
        assert_eq!(part_1(input), 484);
    }

    #[test]
    fn solve_part_1_example() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_1(input), 1227775554);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(2);
        assert_eq!(part_1(&input), 35367539282);
    }

    #[test]
    fn part_2_use_cases() {
        let input = "1-14";
        assert_eq!(part_2(input), 11);

        let input = "384-896";
        assert_eq!(part_2(input), 3330);

        let input = "8897636-9031809";
        assert_eq!(part_2(input), 0);

        let input = "1437387916-1437426347";
        assert_eq!(part_2(input), 1437414374);

        let input = "942-1466";
        assert_eq!(part_2(input), 7059);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!(part_2(input), 4174379265);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(2);
        assert_eq!(part_2(&input), 45814076230);
    }
}
