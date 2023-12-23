use core::panic;
use std::{
    collections::{hash_map::DefaultHasher, VecDeque},
    hash::{Hash, Hasher},
};

use common::matrix::rotate_right;
use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let raw = parse_input(input);
    let tiled = tilt_north(raw);
    calculate_score(&tiled)
}

fn calculate_score(tile: &[Vec<char>]) -> usize {
    tile.iter()
        .rev()
        .enumerate()
        .map(|(index, row)| row.iter().filter(|c| **c == 'O').count() * (index + 1))
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn tilt_north(raw: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = raw;
    let col_size = result[0].len();

    for column in 0..col_size {
        let mut free_rows = VecDeque::new();
        for row in 0..result.len() {
            match result[row][column] {
                '.' => free_rows.push_back(row),
                '#' => free_rows.clear(),
                'O' => {
                    if let Some(free_row) = free_rows.pop_front() {
                        result[free_row][column] = 'O';
                        result[row][column] = '.';
                        free_rows.push_back(row);
                    }
                }
                _ => panic!("Invalid input"),
            }
        }
    }

    result
}

const CYCLE_LENGTH: usize = 1_000_000_000;

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    let mut tile = parse_input(input);
    let mut cache: Vec<(u64, usize)> = Vec::new();
    for cycle in 0..CYCLE_LENGTH {
        tile = perform_cycle(tile);
        let hash = calculate_hash(&tile);
        let cache_hit = cache.iter().find_position(|(h, _)| *h == hash);
        if let Some(cycle_begining) = cache_hit {
            return predict_result(cycle, cycle_begining.0, &cache);
        } else {
            cache.push((hash, calculate_score(&tile)));
        }
    }

    panic!("No solution found");
}

fn perform_cycle(mut tile: Vec<Vec<char>>) -> Vec<Vec<char>> {
    //NORTH
    tile = tilt_north(tile);
    //WEST
    tile = rotate_right(tile);
    tile = tilt_north(tile);
    //SOUTH
    tile = rotate_right(tile);
    tile = tilt_north(tile);
    //EAST
    tile = rotate_right(tile);
    tile = tilt_north(tile);
    // BACK TO BEGINNING
    rotate_right(tile)
}

fn calculate_hash(tile: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    tile.hash(&mut hasher);
    hasher.finish()
}

fn predict_result(cycle: usize, cycle_begining: usize, cache: &[(u64, usize)]) -> usize {
    let cycle_length = cycle - cycle_begining;
    let cycles_left = CYCLE_LENGTH - cycle;
    let last_value_pos = cycles_left % cycle_length;
    cache[last_value_pos + cycle_begining - 1].1
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part_1(input), 136);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day14.txt");
        assert_eq!(part_1(input), 111339);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part_2(input), 64);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day14.txt");
        assert_eq!(part_2(input), 93736);
    }
}
