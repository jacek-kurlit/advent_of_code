use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    parse_input(input).into_iter().map(biggest_pair).sum()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect_vec())
        .collect()
}

fn biggest_pair(digits: Vec<u8>) -> usize {
    let mut first = digits[0];
    let mut second = digits[1];
    //1 2 3 4 5
    //[2 3]
    //[3 4]
    //[4 5]
    for w2 in digits.windows(2).skip(1) {
        if w2[0] > first {
            first = w2[0];
            second = w2[1];
            continue;
        }
        second = second.max(w2[0]).max(w2[1]);
    }
    (first * 10 + second) as usize
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    // [5]
    // 9412378119
    //78119 - init
    // 9
    // 97819
    //
    // 9965514411
    //14411 - init
    // 99655
    //54411
    // 9965
    //55441
    parse_input(input).into_iter().map(biggest_12_digits).sum()
}

fn biggest_12_digits(digits: Vec<u8>) -> u64 {
    let size = 12;
    let tail_start = digits.len() - size;
    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    //init
    digits[tail_start..].iter().for_each(|&d| buffer.push(d));
    for digit in digits[..tail_start].iter().rev() {
        let mut current_digit = *digit;

        for buffer_value in &mut buffer {
            if current_digit >= *buffer_value {
                std::mem::swap(buffer_value, &mut current_digit);
            } else {
                break;
            }
        }
    }
    let mut sum = 0;
    for digit in buffer.iter() {
        sum *= 10;
        sum += *digit as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_1(input), 357);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(3);
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_2(input), 3121910778619);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(3);
        assert_eq!(part_2(&input), 0);
    }
}
