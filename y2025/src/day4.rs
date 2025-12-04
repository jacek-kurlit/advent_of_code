use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let map = parse_input(input);
    count_rolls(map)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec()
}

#[allow(clippy::needless_range_loop)]
fn count_rolls(map: Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    let rows = map.len() as i32;
    let columns = map[0].len() as i32;
    for row in 0..rows {
        for column in 0..columns {
            if map[row as usize][column as usize] == '.' {
                continue;
            }

            let paper_count = map.count_zone(row, column);
            if paper_count < 4 {
                sum += 1;
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let mut rollpepars_map = parse_input(input);
    let mut number_of_removed = count_rolls_and_remove(&mut rollpepars_map);
    let mut sum = number_of_removed;
    while number_of_removed > 0 {
        number_of_removed = count_rolls_and_remove(&mut rollpepars_map);
        sum += number_of_removed;
    }

    sum
}

#[allow(clippy::needless_range_loop)]
fn count_rolls_and_remove(rollpapers_map: &mut Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    let rows = rollpapers_map.len() as i32;
    let columns = rollpapers_map[0].len() as i32;
    for row in 0..rows {
        for column in 0..columns {
            if rollpapers_map[row as usize][column as usize] == '.' {
                continue;
            }
            let paper_count = rollpapers_map.count_zone(row, column);

            if paper_count < 4 {
                rollpapers_map[row as usize][column as usize] = '.';
                sum += 1;
            }
        }
    }
    sum
}

trait RollpaperMap {
    fn is_paper_roll(&self, row: i32, column: i32) -> i32;
    fn count_zone(&self, row: i32, column: i32) -> i32;
}

impl RollpaperMap for Vec<Vec<char>> {
    fn is_paper_roll(&self, row: i32, column: i32) -> i32 {
        let rows = self.len() as i32;
        let columns = self[0].len() as i32;
        if row < 0 || row >= rows || column < 0 || column >= columns {
            return 0;
        }
        if self[row as usize][column as usize] == '@' {
            1
        } else {
            0
        }
    }

    fn count_zone(&self, row: i32, column: i32) -> i32 {
        self.is_paper_roll(row - 1, column - 1)
            + self.is_paper_roll(row - 1, column)
            + self.is_paper_roll(row - 1, column + 1)
            + self.is_paper_roll(row, column - 1)
            + self.is_paper_roll(row, column + 1)
            + self.is_paper_roll(row + 1, column - 1)
            + self.is_paper_roll(row + 1, column)
            + self.is_paper_roll(row + 1, column + 1)
    }
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(4);
        assert_eq!(part_1(&input), 1437);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part_2(input), 43);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(4);
        assert_eq!(part_2(&input), 0);
    }
}
