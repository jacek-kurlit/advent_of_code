use std::collections::HashMap;

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    input.lines().filter(|l| is_nice_string(l)).count()
}

const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOVELS: &str = "aeiou";

fn is_nice_string(line: &str) -> bool {
    if FORBIDDEN.iter().any(|f| line.contains(f)) {
        return false;
    }
    let vovels_count = line.chars().filter(|c| VOVELS.contains(*c)).count();
    if vovels_count < 3 {
        return false;
    }
    line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b)
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| is_nice_string2(l))
        .inspect(|l| {
            dbg!(l);
        })
        .count()
}

fn is_nice_string2(line: &str) -> bool {
    let chars = line.chars().collect_vec();
    if !chars.windows(3).any(|w| w[0] == w[2]) {
        return false;
    }
    let mut counts = HashMap::new();
    let mut prev = (chars[0], chars[1]);
    counts.insert(prev, 1);
    for w in chars.windows(2).skip(1) {
        let pair = (w[0], w[1]);
        if pair == prev {
            continue;
        }
        let c = counts.entry(pair).or_insert(0);
        *c += 1;

        if *c > 1 {
            return true;
        }
        prev = pair;
    }
    false
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(5);
        assert_eq!(part_1(&input), 258);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"qjhvhtzxzqqjkmpb
aaa
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";
        assert_eq!(part_2(input), 2);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(5);
        assert_eq!(part_2(&input), 53);
    }
}
