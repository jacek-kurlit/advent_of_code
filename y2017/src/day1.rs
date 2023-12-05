pub fn task1(input: &str) -> u32 {
    let mut digits = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    digits.push(digits[0]);

    digits
        .windows(2)
        .map(|w| if w[0] == w[1] { w[0] } else { 0 })
        .sum()
}

pub fn task2(input: &str) -> u32 {
    let mut digits = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let halfway = digits.len() / 2;
    digits.extend_from_within(0..halfway);

    digits
        .windows(halfway + 1)
        .map(|w| if w[0] == w[halfway] { w[0] } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solve_task1() {
        assert_eq!(task1("1122"), 3);
        assert_eq!(task1("1111"), 4);
        assert_eq!(task1("1234"), 0);
        assert_eq!(task1("91212129"), 9);
    }

    #[test]
    fn solve_task2() {
        assert_eq!(task2("1212"), 6);
        assert_eq!(task2("123425"), 4);
        assert_eq!(task2("1221"), 0);
        assert_eq!(task2("123123"), 12);
        assert_eq!(task2("12131415"), 4);
    }
}
