use common::{algorithms::polygon::Polygon2, coordinates::Vec2};
use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> i64 {
    let coords = parse_input(input);
    let mut current_max = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let y_len = (coords[i].0 - coords[j].0).abs() + 1;
            let x_len = (coords[i].1 - coords[j].1).abs() + 1;
            let area = y_len * x_len;
            current_max = area.max(current_max);
        }
    }
    current_max
}

#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
    let coords = parse_input(input);
    let polygon = Polygon2::from_tuple(coords);
    let mut current_max = 0;
    let mut queue = Vec::new();
    for i in 0..polygon.len() {
        for j in i + 1..polygon.len() {
            let a = polygon.get_coord(i);
            let b = polygon.get_coord(j);
            let y_len = (a.y - b.y).abs() + 1;
            let x_len = (a.x - b.x).abs() + 1;
            let area = y_len * x_len;
            queue.push((i, j, area));
            if area >= current_max {
                current_max = area;
            }
        }
    }
    for (i, j, area) in queue.into_iter().sorted_unstable_by(|a, b| b.2.cmp(&a.2)) {
        let a = polygon.get_coord(i);
        let b = polygon.get_coord(j);

        if all_points_in_polygon(&polygon, a, b) {
            return area;
        }
    }

    0
}

fn all_points_in_polygon(polygon: &Polygon2<i64>, a: &Vec2<i64>, b: &Vec2<i64>) -> bool {
    let min_x = a.x.min(b.x);
    let min_y = a.y.min(b.y);
    let max_x = a.x.max(b.x);
    let max_y = a.y.max(b.y);
    for y in min_y..=max_y {
        let p1 = Vec2::new(a.x, y);
        let p2 = Vec2::new(b.x, y);
        if !polygon.contains_point(&p1) || !polygon.contains_point(&p2) {
            return false;
        }
    }
    for x in min_x..=max_x {
        let p1 = Vec2::new(x, a.y);
        let p2 = Vec2::new(x, b.y);
        if !polygon.contains_point(&p1) || !polygon.contains_point(&p2) {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| {
            let mut v = l.split(",");
            let y = v.next().unwrap().parse().unwrap();
            let x = v.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_1(input), 50);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(9);
        assert_eq!(part_1(&input), 4748769124);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_2(input), 24);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(9);
        assert_eq!(part_2(&input), 1525991432);
    }
}
