#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|tile| find_reflection_point(tile, 0))
        .sum()
}

fn find_reflection_point(tile: Tile, smuges_count: usize) -> usize {
    find_rows_reflection_point(&tile, smuges_count)
        .or_else(|| find_columns_reflection_point(&tile, smuges_count))
        .expect("Invalid state")
}

fn find_rows_reflection_point(tile: &Tile, smuges_count: usize) -> Option<usize> {
    (0..tile.len() - 1)
        .find(|&row_index| rows_are_reflected(tile, row_index, smuges_count))
        .map(|v| (v + 1) * 100)
}

fn find_columns_reflection_point(rows: &Tile, smuges_count: usize) -> Option<usize> {
    let columns_count = rows[0].len();
    (0..columns_count - 1)
        .find(|&column_index| {
            columns_are_reflected(rows, column_index, columns_count, smuges_count)
        })
        .map(|v| v + 1)
}

fn rows_are_reflected(tile: &Tile, start_index: usize, smuges_count: usize) -> bool {
    let a = (0..=start_index).rev().flat_map(|i| tile[i].iter());
    let b = (start_index + 1..tile.len()).flat_map(|i| tile[i].iter());

    a.zip(b).filter(|(a, b)| a != b).count() == smuges_count
}

fn columns_are_reflected(
    tile: &Tile,
    start_index: usize,
    columns_count: usize,
    smuges_count: usize,
) -> bool {
    let a = (0..=start_index)
        .rev()
        .flat_map(|i| tile.iter().map(move |row| row[i]));
    let b = (start_index + 1..columns_count).flat_map(|i| tile.iter().map(move |row| row[i]));
    a.zip(b).filter(|(a, b)| a != b).count() == smuges_count
}

type Tile = Vec<Vec<char>>;

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile| tile.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|tile| find_reflection_point(tile, 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part_1(input), 405);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day13.txt");
        assert_eq!(part_1(input), 30518);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part_2(input), 400);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day13.txt");
        assert_eq!(part_2(input), 36735);
    }
}
