use std::collections::BTreeMap;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    calculate_galaxies_distances(input, 1)
}

#[allow(dead_code)]
fn part_2(input: &str, distance_size: usize) -> usize {
    calculate_galaxies_distances(input, distance_size - 1)
}

fn calculate_galaxies_distances(input: &str, distance_size: usize) -> usize {
    let expanded_universe = parse_input(input, distance_size);
    expanded_universe
        .values()
        .enumerate()
        .map(|(index, point)| {
            expanded_universe
                .values()
                .skip(index + 1)
                .map(|point2| distance(point, point2))
                .sum::<usize>()
        })
        .sum()
}

type Point = (usize, usize);

fn parse_input(input: &str, distance_size: usize) -> BTreeMap<Point, Point> {
    let unexpanded_galaxy: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxy: BTreeMap<Point, Point> = BTreeMap::new();
    let mut row_offset = 0;
    for (row, row_map) in unexpanded_galaxy.iter().enumerate() {
        let mut galaxy_found = false;
        for (column, point) in row_map.iter().enumerate() {
            if point == &'#' {
                galaxy.insert((row, column), (row + row_offset, column));
                galaxy_found = true;
            }
        }
        if !galaxy_found {
            row_offset += distance_size;
        }
    }

    let mut column_offset = 0;
    for column in 0..unexpanded_galaxy.len() {
        let mut galaxy_found = false;
        for (row, row_map) in unexpanded_galaxy.iter().enumerate() {
            if row_map[column] == '#' {
                galaxy.get_mut(&(row, column)).expect("galaxy not found").1 += column_offset;
                galaxy_found = true;
            }
        }
        if !galaxy_found {
            column_offset += distance_size;
        }
    }

    galaxy
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
