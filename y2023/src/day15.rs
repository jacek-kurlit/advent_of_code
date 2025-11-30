use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, one_of},
    combinator::opt,
    multi::separated_list1,
    IResult, Parser,
};

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    input.trim().split(',').map(calculate_hash).sum()
}

fn calculate_hash(x: &str) -> u32 {
    x.chars().fold(0, |acc, c| {
        let ascii_value = c as u32;
        let acc = acc + ascii_value;
        let acc = acc * 17;
        acc % 256
    })
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    parse_input(input.trim())
        .into_iter()
        .map(|(box_pos, box_)| {
            box_.lens
                .into_iter()
                .enumerate()
                .map(|(lens_pos, lens)| {
                    (box_pos + 1) * (lens_pos as u32 + 1) * lens.focal_length as u32
                })
                .sum::<u32>()
        })
        .sum()
}

fn parse_input(input: &str) -> HashMap<u32, Box> {
    let mut boxes = HashMap::with_capacity(256);
    for opertion in parse_operations(input) {
        match opertion {
            Operation::Add {
                label,
                focal_length,
            } => {
                let hash = calculate_hash(&label);
                let box_ = boxes.entry(hash).or_insert_with(Box::new);
                box_.add(label, focal_length);
            }
            Operation::Remove { label } => {
                let hash = calculate_hash(&label);
                let box_list = boxes.get_mut(&hash);
                if let Some(box_) = box_list {
                    box_.remove(label);
                }
            }
        }
    }
    boxes
}

fn parse_operations(operation: &str) -> Vec<Operation> {
    separated_list1(tag(","), parse_operation)
        .parse(operation)
        .unwrap()
        .1
}

fn parse_operation(operation: &str) -> IResult<&str, Operation> {
    let (rest, label) = alpha1(operation)?;
    let (rest, sign) = one_of("=-")(rest)?;
    let (rest, focal_length) = opt(nom::character::complete::u8).parse(rest)?;
    let operation = match sign {
        '=' => Operation::Add {
            label: label.to_string(),
            focal_length: focal_length.expect("Missing focal length for add operation"),
        },
        '-' => Operation::Remove {
            label: label.to_string(),
        },
        _ => panic!("Invalid operation"),
    };
    Ok((rest, operation))
}

type FocalLength = u8;
enum Operation {
    Add {
        label: String,
        focal_length: FocalLength,
    },
    Remove {
        label: String,
    },
}

struct Box {
    lens: VecDeque<Lens>,
}

struct Lens {
    label: String,
    focal_length: FocalLength,
}

impl Box {
    fn new() -> Box {
        Box {
            lens: VecDeque::new(),
        }
    }

    fn add(&mut self, label: String, focal_length: FocalLength) {
        let current_lens = self.lens.iter_mut().find(|l| l.label == label);
        if let Some(lens) = current_lens {
            lens.focal_length = focal_length;
        } else {
            self.lens.push_back(Lens {
                label,
                focal_length,
            });
        }
    }

    fn remove(&mut self, label: String) {
        self.lens.retain(|l| l.label != label);
    }
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_1(input), 1320);
    }

    #[test]
    fn part_1_extra_tests() {
        assert_eq!(part_1("rn"), 0);
        assert_eq!(part_1("qp"), 1);
        assert_eq!(part_1("pc"), 3);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day15.txt");
        assert_eq!(part_1(input), 514025);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_2(input), 145);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day15.txt");
        assert_eq!(part_2(input), 244461);
    }
}
