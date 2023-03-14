pub fn task1(input: String) -> u32 {
    input
        .lines()
        .map(|row| {
            let row_numbers = prase_row(row);
            row_numbers.iter().max().copied().unwrap_or_default()
                - row_numbers.iter().min().copied().unwrap_or_default()
        })
        .sum()
}

fn prase_row(row: &str) -> Vec<u32> {
    row.split(' ')
        .map(|s| s.parse::<u32>().expect("Should be parsable"))
        .collect::<Vec<u32>>()
}

pub fn task2(input: String) -> u32 {
    input
        .lines()
        .map(|row| {
            let row_numbers = prase_row(row);
            let x = row_numbers
                .windows(2)
                .map(|w| (w[0].max(w[1]), w[0].min(w[1])))
                .filter(|(max, min)| max % min == 0)
                .map(|(max, min)| max / min)
                .next()
                .unwrap_or_default();
            x
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5 1 9 5
7 5 3
2 4 6 8";

    #[test]
    fn should_solve_task1() {
        assert_eq!(task1(INPUT.to_string()), 18);
    }

    #[test]
    fn should_solve_task2() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!(task2(input.to_string()), 9);
    }
}
