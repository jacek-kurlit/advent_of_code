pub fn task1(input: String) -> u32 {
    println!("Input: {}", input);
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
            for i in 0..row_numbers.len() {
                for j in i + 1..row_numbers.len() {
                    let max = row_numbers[i].max(row_numbers[j]);
                    let min = row_numbers[i].min(row_numbers[j]);
                    if max % min == 0 {
                        return (max, min);
                    }
                }
            }
            (0, 0)
        })
        .map(|(max, min)| max / min)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_task1() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!(task1(input.to_string()), 18);
    }

    #[test]
    fn should_solve_task2() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
        println!("dada");
        assert_eq!(task2(input.to_string()), 9);
    }
}
