use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect();
    let mut lines = input.into_iter();
    let start = lines
        .next()
        .unwrap()
        .iter()
        .position(|c| *c == 'S')
        .unwrap();
    let mut splits = 0;
    let mut beams = HashSet::new();
    beams.insert(start);
    for line in lines {
        let mut new_beams = HashSet::new();
        for beam in beams {
            if line[beam] == '^' {
                splits += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }

    splits
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let manifolds: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect();
    let start = manifolds
        .first()
        .unwrap()
        .iter()
        .position(|c| *c == 'S')
        .unwrap();
    count_splits(&manifolds[1..], start, 1, &mut HashMap::new())
}

fn count_splits(
    manifolds: &[Vec<char>],
    pos: usize,
    row: usize,
    memory: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row == manifolds.len() - 1 {
        return 1;
    }
    if let Some(splits) = memory.get(&(pos, row)) {
        return *splits;
    }
    if manifolds[row][pos] == '.' {
        let mut new_row = row + 1;
        while manifolds[new_row][pos] == '.' && new_row < manifolds.len() - 1 {
            new_row += 1;
        }
        let splits = count_splits(manifolds, pos, new_row, memory);
        memory.insert((pos, new_row), splits);
        return splits;
    }
    // ^ case
    let left_splits = count_splits(manifolds, pos - 1, row + 1, memory);
    memory.insert((pos - 1, row + 1), left_splits);

    let right_splists = count_splits(manifolds, pos + 1, row + 1, memory);
    memory.insert((pos + 1, row + 1), right_splists);
    left_splits + right_splists
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(7);
        assert_eq!(part_1(&input), 1619);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part_2(input), 40);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(7);
        assert_eq!(part_2(&input), 23607984027985);
    }
}
