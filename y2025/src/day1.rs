#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let mut result = 0;
    let mut pos: i16 = 50;
    for rotation in parse_input(input) {
        pos += rotation;
        if pos < 0 {
            pos = 100 + (pos % 100);
        } else if pos > 100 {
            pos %= 100;
        }

        if pos.abs() == 100 {
            pos = 0;
        }

        if pos == 0 {
            result += 1;
        }
    }
    result
}

fn parse_input(input: &str) -> Vec<i16> {
    input
        .trim()
        .split("\n")
        .map(|rotation| {
            let rotation = rotation.trim();
            let direction = rotation.chars().next().expect("Missing direction");
            let value: i16 = rotation[1..].parse().expect("Missing value");
            if direction == 'R' { value } else { -value }
        })
        .collect()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let mut result = 0;
    let mut pos: i16 = 50;
    for rotation in parse_input(input) {
        result += (rotation.abs() / 100) as usize;
        let mut new_value = pos + (rotation % 100);
        if new_value <= 0 {
            let offset = if pos != 0 { 1 } else { 0 };
            result += offset;
            new_value = if new_value == 0 {
                0
            } else {
                100 + (new_value % 100)
            };
        } else if new_value >= 100 {
            result += 1;
            new_value %= 100;
        }
        pos = new_value;
    }
    result
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"L68
         L30
         R48
         L5
         R60
         L55
         L1
         L99
         R14
         L82";
        assert_eq!(part_1(input), 3);

        let input = r"L150
R15
R85
R593
L351";
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(1);
        assert_eq!(part_1(&input), 984);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"L68
         L30
         R48
         L5
         R60
         L55
         L1
         L99
         R14
         L82";
        assert_eq!(part_2(input), 6);
    }

    #[test]
    fn solve_part_2_use_cases() {
        let input = r"L150
R15
R85
R593
L351";
        assert_eq!(part_2(input), 11);
    }
    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(1);
        assert_eq!(part_2(&input), 0);
    }
}
