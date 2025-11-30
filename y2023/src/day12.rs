use std::collections::HashMap;

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    parse_input(input)
        .into_iter()
        .map(find_all_arrangements)
        .sum()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    input.len() as u32
}

fn find_all_arrangements(mut record: ConditionRecord) -> u32 {
    backtrack(
        &record.unknown_positions,
        &mut record.springs,
        &record.damage_ranges,
    )
}

fn backtrack(unknown_positions: &[usize], springs: &mut [char], damage_ranges: &[u32]) -> u32 {
    let conditon = check_condition(springs, damage_ranges);
    match conditon {
        Condition::Valid => 1,
        Condition::Invalid => 0,
        Condition::Incomplete => {
            let mut count = 0;
            let next_unknown = unknown_positions.first().expect("Invalid state");
            springs[*next_unknown] = '#';
            count += backtrack(&unknown_positions[1..], springs, damage_ranges);
            springs[*next_unknown] = '.';
            count += backtrack(&unknown_positions[1..], springs, damage_ranges);
            springs[*next_unknown] = '?';
            count
        }
    }
}

enum Condition {
    Valid,
    Invalid,
    Incomplete,
}

//???.### 1,1,3
//#?????.### 2,3
//FIXME: to optimize
fn check_condition(springs: &[char], damage_ranges: &[u32]) -> Condition {
    let parts = count_parts(springs);
    if let Some(broken_parts) = parts.get(&'#') {
        if broken_parts.len() > damage_ranges.len() {
            return Condition::Invalid;
        }
        if broken_parts.len() == damage_ranges.len()
            && broken_parts_are_set(broken_parts, damage_ranges)
        {
            return Condition::Valid;
            //TODO: here add check that this is invalid if chain is broken and there is no '?' in the chain
        }
        // #????.### 2,1,3
        // for (range, expected_size) in broken_parts.iter().zip(damage_ranges) {
        //     if range.len() as u32 > *expected_size {
        //         return Condition::Invalid;
        //     }
        // }
    }

    if !parts.contains_key(&'?') {
        return Condition::Invalid;
    }

    Condition::Incomplete
}

fn broken_parts_are_set(bp: &[Vec<usize>], damage_ranges: &[u32]) -> bool {
    bp.iter()
        .zip(damage_ranges)
        .all(|(part, range)| part.len() as u32 == *range)
}

fn count_parts(springs: &[char]) -> HashMap<char, Vec<Vec<usize>>> {
    springs
        .iter()
        .enumerate()
        .chunk_by(|c| *c.1)
        .into_iter()
        .fold(HashMap::new(), |mut acc, (key, group)| {
            let positions = acc.entry(key).or_default();
            positions.push(group.map(|(pos, _)| pos).collect());
            acc
        })
}

fn parse_input(input: &str) -> Vec<ConditionRecord> {
    input
        .lines()
        .map(|line| {
            let data = line.split_once(' ').unwrap();
            ConditionRecord {
                unknown_positions: data
                    .0
                    .chars()
                    .enumerate()
                    .filter_map(|(pos, c)| if c == '?' { Some(pos) } else { None })
                    .collect(),
                springs: data.0.chars().collect(),
                damage_ranges: data.1.split(',').map(|s| s.parse().unwrap()).collect(),
            }
        })
        .collect()
}

struct ConditionRecord {
    unknown_positions: Vec<usize>,
    springs: Vec<char>,
    damage_ranges: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day12::count_parts;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn part_1_extra_tests() {
        assert_eq!(part_1("???.### 1,1,3"), 1);
        assert_eq!(part_1("#?????.### 2,3"), 1);
        assert_eq!(part_1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part_1("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day12.txt");
        assert_eq!(part_1(input), 7344);
    }

    #[test]
    fn test_grouping_by() {
        let input: Vec<char> = "#??#?.###".chars().collect();
        let parts = count_parts(&input);
        dbg!(&parts);
        assert_eq!(parts, HashMap::default());
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part_2(input), 525152);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day12.txt");
        assert_eq!(part_2(input), 0);
    }
}
