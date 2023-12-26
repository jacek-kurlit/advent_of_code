use common::coordinates::{Vec2, Vec3};
use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str, min_val: f64, max_val: f64) -> usize {
    parse_input(input)
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.intersection_point(b))
        .filter(|point| point.bounded_within(min_val, max_val))
        .count()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    input.len() as u32
}

//19, 13, 30 @ -2,  1, -2
fn parse_input(input: &str) -> Vec<LinearTrajectory2> {
    input.lines().map(parse_trajectory).collect()
}

fn parse_trajectory(input: &str) -> LinearTrajectory2 {
    let (pos, vel) = input.split_once('@').unwrap();
    let position = parse_vector(pos);
    let velocity = parse_vector(vel);
    LinearTrajectory2::new(position.into(), velocity.into())
}

fn parse_vector(input: &str) -> Vec3<f64> {
    input
        .split(',')
        .map(|s| s.trim())
        .map(|v| v.parse::<f64>().unwrap())
        .collect_tuple()
        .map(|(x, y, z)| Vec3::new(x, y, z))
        .unwrap()
}

#[derive(Debug)]
pub struct LinearTrajectory2 {
    pub start: Vec2<f64>,
    pub velocity: Vec2<f64>,
}

impl LinearTrajectory2 {
    pub fn new(start: Vec2<f64>, velocity: Vec2<f64>) -> Self {
        Self { start, velocity }
    }

    // x1 + dx1 * t = x2 + dx2 * s
    // y1 + dy1 * t = y2 + dy2 * s
    // t = (x2 + dx2 * s - x1) / dx1
    // s = a/b
    // a = y1dx1 + dy1x2 - dy1x1 - y2dy1dx1
    // b = dy2dx1 - dy1dx2
    // where
    // b != 0
    // dx1 != 0
    pub fn intersection_point(&self, other: &Self) -> Option<Vec2<f64>> {
        dbg!(self, other);
        if other.velocity.x == 0.0 {
            return None;
        }
        let b = other.velocity.y * self.velocity.x - self.velocity.y * other.velocity.x;
        if b == 0.0 {
            return None;
        }

        let a = self.start.y * self.velocity.x + self.velocity.y * other.start.x
            - self.velocity.y * self.start.x
            - other.start.y * self.velocity.y * self.velocity.x;
        let s = a / b;
        let t = (other.start.x + other.velocity.x * s - self.start.x) / self.velocity.x;
        let result = Vec2::new(
            self.start.x + self.velocity.x * t,
            self.start.y + self.velocity.y * t,
        );
        dbg!(result);
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(part_1(input, 7.0, 27.0), 2);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day24.txt");
        assert_eq!(part_1(input, 200000000000000.0, 400000000000000.0), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day24.txt");
        assert_eq!(part_2(input), 0);
    }
}
