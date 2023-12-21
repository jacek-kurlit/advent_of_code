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

//???.### 1,1,3
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

//NOTE: if we have all damage ranges covered and still there are '?' then we could cheat and return
//remaning numebr of '?' as rest of combinations because we can only use '.' for missing parts to
//be valid
fn check_condition(springs: &[char], damage_ranges: &[u32]) -> Condition {
    //???.### 1,1,3
    let mut count = 0;
    for spring in springs {

    }
    Condition::Incomplete
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
        assert_eq!(part_1("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day12.txt");
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day12.txt");
        assert_eq!(part_2(input), 0);
    }
}
