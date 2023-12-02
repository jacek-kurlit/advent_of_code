const REDS: u32 = 12;
const GREENS: u32 = 13;
const BLUES: u32 = 14;

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_results)
        .filter(is_valid)
        .map(|r| r.id)
        .sum()
}

fn is_valid(results: &ParsedResults) -> bool {
    results.reds.iter().all(|&red| red <= REDS)
        && results.greens.iter().all(|&green| green <= GREENS)
        && results.blues.iter().all(|&blue| blue <= BLUES)
}

struct ParsedResults {
    id: u32,
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>,
}

fn parse_results(line: &str) -> ParsedResults {
    let (game, results) = line.split_once(':').unwrap();
    let mut parsed_results = ParsedResults {
        id: parse_game_id(game),
        reds: Vec::new(),
        greens: Vec::new(),
        blues: Vec::new(),
    };
    for result in results.trim().replace(';', ",").split(',') {
        match result.trim().split(' ').collect::<Vec<&str>>().as_slice() {
            [count, "red"] => {
                parsed_results.reds.push(count.parse::<u32>().unwrap());
            }
            [count, "green"] => {
                parsed_results.greens.push(count.parse::<u32>().unwrap());
            }
            [count, "blue"] => {
                parsed_results.blues.push(count.parse::<u32>().unwrap());
            }
            _ => panic!("unexpected result: {}", result),
        }
    }
    parsed_results
}

fn parse_game_id(game: &str) -> u32 {
    game.replace("Game ", "")
        .replace(':', "")
        .parse::<u32>()
        .expect("game id should be a number")
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_results)
        .map(|results| {
            results.reds.iter().max().unwrap()
                * results.greens.iter().max().unwrap()
                * results.blues.iter().max().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day2.txt");
        assert_eq!(part_1(input), 2061);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(input), 2286);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day2.txt");
        assert_eq!(part_2(input), 72596);
    }
}
