use std::collections::{BTreeMap, HashMap};

use common::coordinates::Vec3;
use itertools::Itertools;

struct JunctionBox {
    position: Vec3<i64>,
}

#[allow(dead_code)]
fn part_1(input: &str, max_connections: usize) -> u64 {
    let junction_boxes = parse_input(input);
    let distances = calculate_distances(&junction_boxes);
    let mut created_connections = 0;
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut box_to_group: HashMap<usize, usize> = HashMap::new();

    let mut next_lowest = distances.into_values();
    let mut unique_group_id = 0;
    while created_connections < max_connections {
        let (a_index, b_index) = next_lowest.next().unwrap();
        created_connections += 1;
        match (
            box_to_group.get(&a_index).copied(),
            box_to_group.get(&b_index).copied(),
        ) {
            (None, None) => {
                let group = unique_group_id;
                unique_group_id += 1;
                box_to_group.insert(a_index, group);
                box_to_group.insert(b_index, group);
                groups.insert(group, vec![a_index, b_index]);
            }
            (None, Some(b_group)) => {
                box_to_group.insert(a_index, b_group);
                groups.get_mut(&b_group).unwrap().push(a_index);
            }
            (Some(a_group), None) => {
                box_to_group.insert(b_index, a_group);
                groups.get_mut(&a_group).unwrap().push(b_index);
            }
            (Some(a_group), Some(b_group)) => {
                if a_group == b_group {
                    continue;
                }
                let all_b_values = groups.remove(&b_group).unwrap();
                box_to_group.insert(b_index, a_group);
                let all_a_values = groups.get_mut(&a_group).unwrap();
                all_b_values.iter().for_each(|b| {
                    box_to_group.entry(*b).and_modify(|e| *e = a_group);
                    all_a_values.push(*b);
                });
            }
        }
    }
    groups
        .values_mut()
        .map(|g| g.len() as u64)
        .sorted_unstable()
        .rev()
        .take(3)
        .reduce(|acc, v| acc * v)
        .unwrap()
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|l| {
            let mut values = l.trim().split(",");
            let as_val = |v: Option<&str>| v.unwrap().parse().unwrap();
            let position = Vec3::new(
                as_val(values.next()),
                as_val(values.next()),
                as_val(values.next()),
            );
            JunctionBox { position }
        })
        .collect_vec()
}

fn calculate_distances(input: &[JunctionBox]) -> BTreeMap<i64, (usize, usize)> {
    let mut reslut = BTreeMap::new();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let a = input[i].position;
            let b = input[j].position;
            let distance = (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2);
            reslut.insert(distance, (i, j));
        }
    }

    reslut
}

#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
    let junction_boxes = parse_input(input);
    let distances = calculate_distances(&junction_boxes);
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut box_to_group: HashMap<usize, usize> = HashMap::new();

    let mut next_lowest = distances.into_values();
    let mut unique_group_id = 0;
    let mut latest = (0, 0);
    while !(groups.len() == 1
        && groups.values().next().unwrap_or(&vec![]).len() == junction_boxes.len())
    {
        let (a_index, b_index) = next_lowest.next().unwrap();
        match (
            box_to_group.get(&a_index).copied(),
            box_to_group.get(&b_index).copied(),
        ) {
            (None, None) => {
                let group = unique_group_id;
                unique_group_id += 1;
                box_to_group.insert(a_index, group);
                box_to_group.insert(b_index, group);
                groups.insert(group, vec![a_index, b_index]);
            }
            (None, Some(b_group)) => {
                box_to_group.insert(a_index, b_group);
                groups.get_mut(&b_group).unwrap().push(a_index);
            }
            (Some(a_group), None) => {
                box_to_group.insert(b_index, a_group);
                groups.get_mut(&a_group).unwrap().push(b_index);
            }
            (Some(a_group), Some(b_group)) => {
                if a_group == b_group {
                    continue;
                }
                let all_b_values = groups.remove(&b_group).unwrap();
                box_to_group.insert(b_index, a_group);
                let all_a_values = groups.get_mut(&a_group).unwrap();
                all_b_values.iter().for_each(|b| {
                    box_to_group.entry(*b).and_modify(|e| *e = a_group);
                    all_a_values.push(*b);
                });
            }
        }
        latest = (a_index, b_index);
    }
    junction_boxes[latest.0].position.x * junction_boxes[latest.1].position.x
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part_1(input, 10), 40);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(8);
        assert_eq!(part_1(&input, 1000), 97384);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part_2(input), 25272);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(8);
        assert_eq!(part_2(&input), 9003685096);
    }
}
