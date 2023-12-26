use core::str;
use std::{collections::HashMap, u64};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, char, line_ending, one_of, u32},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    let (workflows, ratings) = parse_input(input);
    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name, workflow.rules))
        .collect::<HashMap<_, _>>();
    let mut total = 0;
    for rating in ratings {
        total += apply_rules(&rating, "in", &workflows).unwrap_or(0);
    }
    total
}

fn apply_rules(rating: &Rating, workflow_name: &str, workflows: &Workflows) -> Option<u32> {
    let workflow_rules = workflows
        .get(workflow_name)
        .unwrap_or_else(|| panic!("No workflow named '{}'", workflow_name));

    for rule in workflow_rules {
        let next_workflow = rule.next_label(rating);
        match next_workflow {
            Some(Label::A) => {
                return Some(rating.total());
            }
            Some(Label::R) => return None,
            Some(Label::Workflow(workflow_name)) => {
                return apply_rules(rating, workflow_name, workflows);
            }
            _ => {}
        }
    }
    panic!("Flow didn't end with A or R");
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    input.len() as u64
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<Rating>) {
    let (workflows, ratings) = input.split_once("\n\n").expect("Invalid input");
    (parse_workflows(workflows), parse_ratings(ratings))
}

fn parse_workflows(workflows: &str) -> Vec<Workflow> {
    separated_list1(line_ending, parse_workflow)(workflows)
        .expect("Failed to parse workflows")
        .1
}

// rfg{s<537:gd,x>2440:R,A}
fn parse_workflow(workflow: &str) -> IResult<&str, Workflow> {
    let (rest, workflow_name) = alpha1(workflow)?;
    let (rest, rules) = delimited(char('{'), parse_rules, char('}'))(rest)?;
    Ok((
        rest,
        Workflow {
            name: workflow_name.to_string(),
            rules,
        },
    ))
}

fn parse_rules(rules: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(char(','), alt((parse_operation_rule, parse_jump_rule)))(rules)
}

fn parse_operation_rule(rule: &str) -> IResult<&str, Rule> {
    let (rest, rating_name) = one_of("xmas")(rule)?;
    let (rest, operation) = one_of("<>")(rest)?;
    let (rest, value) = u32(rest)?;
    let (rest, next_label) = preceded(char(':'), alpha1)(rest)?;
    let rule = match operation {
        '<' => Rule::LessThan {
            rating_name: rating_name.to_string(),
            value,
            next_label: Label::new(next_label),
        },
        '>' => Rule::GreaterThan {
            rating_name: rating_name.to_string(),
            value,
            next_label: Label::new(next_label),
        },
        _ => unreachable!(),
    };
    Ok((rest, rule))
}

fn parse_jump_rule(rule: &str) -> IResult<&str, Rule> {
    let (rest, workflow_name) = alpha1(rule)?;
    Ok((
        rest,
        Rule::JumpTo {
            next_label: Label::new(workflow_name),
        },
    ))
}

fn parse_ratings(ratings: &str) -> Vec<Rating> {
    //{x=787,m=2655,a=1222,s=2876}
    separated_list1(
        complete::line_ending,
        delimited(char('{'), parse_rating, char('}')),
    )(ratings)
    .expect("Failed to parse ratings")
    .1
}
// x=787,m=2655,a=1222,s=2876}
fn parse_rating(rating: &str) -> IResult<&str, Rating> {
    let (rest, x) = delimited(tag("x="), u32, char(','))(rating)?;
    let (rest, m) = delimited(tag("m="), u32, char(','))(rest)?;
    let (rest, a) = delimited(tag("a="), u32, char(','))(rest)?;
    let (rest, s) = preceded(tag("s="), u32)(rest)?;
    Ok((rest, Rating { x, m, a, s }))
}

type Workflows = HashMap<String, Vec<Rule>>;

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Label {
    A,
    R,
    Workflow(String),
}

impl Label {
    fn new(label: &str) -> Self {
        match label {
            "A" => Label::A,
            "R" => Label::R,
            _ => Label::Workflow(label.to_string()),
        }
    }
}

#[derive(Debug)]
enum Rule {
    LessThan {
        rating_name: String,
        value: u32,
        next_label: Label,
    },

    GreaterThan {
        rating_name: String,
        value: u32,
        next_label: Label,
    },
    JumpTo {
        next_label: Label,
    },
}

impl Rule {
    fn next_label(&self, rating: &Rating) -> Option<&Label> {
        match self {
            Rule::LessThan {
                rating_name,
                value,
                next_label,
            } if rating.find_value(rating_name) < *value => Some(next_label),
            Rule::GreaterThan {
                rating_name,
                value,
                next_label,
            } if rating.find_value(rating_name) > *value => Some(next_label),
            Rule::JumpTo { next_label } => Some(next_label),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Rating {
    fn find_value(&self, name: &str) -> u32 {
        match name {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid rating name"),
        }
    }

    fn total(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part_1(input), 19114);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day19.txt");
        assert_eq!(part_1(input), 495298);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part_2(input), 167409079868000);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day19.txt");
        assert_eq!(part_2(input), 0);
    }
}
