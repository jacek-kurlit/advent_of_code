use std::collections::HashMap;

use common::algorithms::lcm::lcm_of_n_numbers;

#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    let parsed_network = parse_input(input);
    road_length(
        parsed_network.nodes.get("AAA").expect("No starting node"),
        &parsed_network,
        |node| node == "ZZZ",
    )
}

fn road_length(
    node: &Node,
    parsed_network: &ParsedNetwork,
    end_node_predicate: fn(&String) -> bool,
) -> u64 {
    let mut steps = 0;
    let mut current_node = node;
    for instruction in parsed_network.instructions.iter().cycle() {
        steps += 1;
        let next_node = match instruction {
            'L' => &current_node.0,
            'R' => &current_node.1,
            _ => panic!("Invalid instruction {instruction}"),
        };
        if end_node_predicate(next_node) {
            break;
        }
        current_node = parsed_network.nodes.get(next_node).expect("Invalid node");
    }
    steps
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    let parsed_network = parse_input(input);
    let loads_length = parsed_network
        .nodes
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, node)| road_length(node, &parsed_network, |node| node.ends_with('Z')))
        .collect::<Vec<u64>>();
    lcm_of_n_numbers(&loads_length)
}

fn parse_input(input: &str) -> ParsedNetwork {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect();
    lines.next();
    let nodes = lines
        .map(|line| {
            let (from, assignment) = line.split_once(" = ").expect("Invalid input");
            let assigment = assignment.replace(['(', ')'], "");
            let (left, right) = assigment.split_once(',').expect("Invalid input");
            (
                from.trim().to_string(),
                (left.trim().to_string(), right.trim().to_string()),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    ParsedNetwork {
        instructions,
        nodes,
    }
}

type Node = (String, String);
struct ParsedNetwork {
    instructions: Vec<char>,
    nodes: HashMap<String, Node>,
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_1(input), 2);

        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_1(input), 6);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day8.txt");
        assert_eq!(part_1(input), 20777);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part_2(input), 6);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day8.txt");
        assert_eq!(part_2(input), 13289612809129);
    }
}
