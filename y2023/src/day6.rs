#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    parse_races(input)
        .into_iter()
        .map(calculate_number_of_winning_ways)
        .product::<u64>()
}

fn calculate_number_of_winning_ways(race: ParsedRace) -> u64 {
    let mut winning_ways = 0;
    for hold_for in 1..race.time {
        let travel_distance = hold_for * (race.time - hold_for);
        if travel_distance > race.distance {
            winning_ways += 1;
        }
    }
    winning_ways
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    let input = input.replace(' ', "");
    part_1(&input)
}

fn parse_races(input: &str) -> Vec<ParsedRace> {
    let mut lines = input.lines();
    let times = parse_as_numbers(lines.next());
    let distances = parse_as_numbers(lines.next());
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| ParsedRace { time, distance })
        .collect()
}

fn parse_as_numbers(input: Option<&str>) -> Vec<u64> {
    input
        .expect("Invalid input")
        .split_once(':')
        .expect("Invalid input")
        .1
        .split_whitespace()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.parse::<u64>().expect("Invalid input"))
        .collect::<Vec<u64>>()
}

struct ParsedRace {
    time: u64,
    distance: u64,
}

#[cfg(test)]
mod tests {
    use crate::day6::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_1(input), 288);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day6.txt");
        assert_eq!(part_1(input), 1195150);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_2(input), 71503);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day6.txt");
        assert_eq!(part_2(input), 42550411);
    }
}
