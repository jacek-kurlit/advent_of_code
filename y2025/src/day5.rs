use std::{collections::VecDeque, ops::RangeInclusive};

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let input = parse_input(input);
    input
        .ids_to_check
        .into_iter()
        .filter(|id| input.fresh_ranges.iter().any(|ranege| ranege.contains(id)))
        .count()
}

struct Input {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ids_to_check: Vec<u64>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.trim().lines();
    let mut fresh_ranges = vec![];
    let mut line = lines.next().unwrap().trim();

    while !line.is_empty() {
        let mut values = line.split("-");
        let a: u64 = values.next().unwrap().parse().unwrap();
        let b: u64 = values.next().unwrap().parse().unwrap();
        fresh_ranges.push(a..=b);
        line = lines.next().unwrap_or_default();
    }

    Input {
        fresh_ranges,
        ids_to_check: lines.map(|id| id.parse::<u64>().unwrap()).collect_vec(),
    }
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let input = parse_input(input);
    let merged = merge_overlaping(input.fresh_ranges);
    merged.into_iter().map(|range| range.count()).sum()
}

fn merge_overlaping(mut ranges: Vec<RangeInclusive<u64>>) -> VecDeque<RangeInclusive<u64>> {
    //1-5
    //3-150
    //
    //1-150
    //4-100
    //
    // 3-5
    // 10-14
    // 12-18
    // 16-20
    //
    // 1-4
    // 2-3
    // 3-4
    // 5-9
    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));
    let mut result = VecDeque::new();
    let mut range_inter = ranges.into_iter();
    result.push_front(range_inter.next().unwrap());
    for range in range_inter {
        let last = result.pop_back().unwrap();
        if last.end() >= range.start() {
            let end = range.end().max(last.end());
            result.push_back(*last.start()..=*end);
        } else {
            result.push_back(last);
            result.push_back(range);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part_1(input), 3);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(5);
        assert_eq!(part_1(&input), 615);
    }

    #[test]
    fn part_2_use_cases() {
        let input = r"1-4
2-3
3-4
5-9

1";
        assert_eq!(part_2(input), 4 + 5);

        let input = r"1-3
15-16
3-10
11-20

1";
        assert_eq!(part_2(input), 11 + 10);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part_2(input), 14);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(5);
        assert_eq!(part_2(&input), 353716783056994);
    }
}
