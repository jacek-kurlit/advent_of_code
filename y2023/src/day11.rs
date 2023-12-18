use std::collections::BTreeMap;

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    calculate_galaxies_distances(input, 1)
}

#[allow(dead_code)]
fn part_2(input: &str, distance_size: usize) -> usize {
    calculate_galaxies_distances(input, distance_size - 1)
}

fn calculate_galaxies_distances(input: &str, distance_size: usize) -> usize {
    parse_galaxies(input, distance_size)
        .values()
        .combinations(2)
        .map(|points| distance(points[0], points[1]))
        .sum()
}

type Point = (usize, usize);

fn parse_galaxies(input: &str, distance_size: usize) -> BTreeMap<Point, Point> {
    let galaxy: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut expanded_galaxy: BTreeMap<Point, Point> = BTreeMap::new();
    expand_on_rows(&galaxy, &mut expanded_galaxy, distance_size);
    expand_on_columns(galaxy, &mut expanded_galaxy, distance_size);

    expanded_galaxy
}

fn expand_on_rows(
    galaxy: &[Vec<char>],
    expanded_galaxy: &mut BTreeMap<(usize, usize), (usize, usize)>,
    distance_size: usize,
) {
    let mut row_offset = 0;
    for (row, row_map) in galaxy.iter().enumerate() {
        let mut galaxy_found = false;
        for (column, point) in row_map.iter().enumerate() {
            if point == &'#' {
                expanded_galaxy.insert((row, column), (row + row_offset, column));
                galaxy_found = true;
            }
        }
        if !galaxy_found {
            row_offset += distance_size;
        }
    }
}

fn expand_on_columns(
    galaxy: Vec<Vec<char>>,
    expanded_galaxy: &mut BTreeMap<(usize, usize), (usize, usize)>,
    distance_size: usize,
) {
    let mut column_offset = 0;
    for column in 0..galaxy.len() {
        let mut galaxy_found = false;
        for (row, row_map) in galaxy.iter().enumerate() {
            if row_map[column] == '#' {
                expanded_galaxy
                    .get_mut(&(row, column))
                    .expect("galaxy not found")
                    .1 += column_offset;
                galaxy_found = true;
            }
        }
        if !galaxy_found {
            column_offset += distance_size;
        }
    }
}

fn distance(point: &Point, point2: &Point) -> usize {
    let row_diff = point2.0 - point.0;
    let column_diff = point2.1 as i32 - point.1 as i32;
    row_diff + column_diff.unsigned_abs() as usize
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part_1(input), 374);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day11.txt");
        assert_eq!(part_1(input), 9681886);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part_2(input, 10), 1030);
        assert_eq!(part_2(input, 100), 8410);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day11.txt");
        assert_eq!(part_2(input, 1_000_000), 791134099634);
    }
}
