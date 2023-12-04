#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    input.lines().map(part1_digits_sum).sum()
}

fn part1_digits_sum(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    first_plus_last(digits)
}

fn first_plus_last(digits: Vec<u32>) -> u32 {
    let first = digits.first().copied().unwrap();
    let last = digits.last().copied().unwrap();
    first * 10 + last
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    input.lines().map(part2_digits_sum).sum()
}

fn part2_digits_sum(line: &str) -> u32 {
    let mut chars = line.chars().collect::<Vec<char>>();
    let first = find_first_digit(chars.as_slice());
    // NOTE: reason for reverse is twoone should be 21 not 22!
    // which means I should move single char forward each time instead of whole number string
    // If I did so I would not need to reverse the chars...
    chars.reverse();
    let last = find_first_digit(chars.as_slice());
    first * 10 + last
}

fn find_first_digit(input: &[char]) -> u32 {
    match input {
        [c, ..] if c.is_ascii_digit() => c.to_digit(10).unwrap(),
        ['o', 'n', 'e', ..] | ['e', 'n', 'o', ..] => 1,
        ['t', 'w', 'o', ..] | ['o', 'w', 't', ..] => 2,
        ['t', 'h', 'r', 'e', 'e', ..] | ['e', 'e', 'r', 'h', 't', ..] => 3,
        ['f', 'o', 'u', 'r', ..] | ['r', 'u', 'o', 'f', ..] => 4,
        ['f', 'i', 'v', 'e', ..] | ['e', 'v', 'i', 'f', ..] => 5,
        ['s', 'i', 'x', ..] | ['x', 'i', 's', ..] => 6,
        ['s', 'e', 'v', 'e', 'n', ..] | ['n', 'e', 'v', 'e', 's', ..] => 7,
        ['e', 'i', 'g', 'h', 't', ..] | ['t', 'h', 'g', 'i', 'e', ..] => 8,
        ['n', 'i', 'n', 'e', ..] | ['e', 'n', 'i', 'n', ..] => 9,
        [_, rest @ ..] => find_first_digit(rest),
        [] => panic!("no digit found"),
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(part_1(input), 142);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day1.txt");
        assert_eq!(part_1(input), 54632);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part_2(input), 281)
    }

    use rstest::rstest;

    //NOTE: this is not working with neotest-rust yet https://github.com/rouge8/neotest-rust/pull/57
    #[rstest]
    #[case("one1two2three3four4five5six6seven7eight8nine9", 19)]
    #[case("9twotwoqlvkrkhjthree44shvjxkpjgzgphgprflvn", 94)]
    #[case("zcddqkhkjlfive4onexkdggcbfbqzxhfxqnb6", 56)]
    #[case("1fourtwo", 12)]
    #[case("eightone4", 84)]
    #[case("threetvlsjbmnfive83three", 33)]
    #[case("fiveeng8jjjttwone", 51)]
    fn part_2_extra_examples(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part_2(input), expected);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day1.txt");
        assert_eq!(part_2(input), 54019);
    }
}
