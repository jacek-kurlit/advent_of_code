#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    let cards = parse_cards(input);
    cards
        .iter()
        .map(|card| card.number_of_hits())
        .filter(|&count| count > 0)
        .map(|count| 1 << (count - 1))
        .sum()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    let cards = parse_cards(input);
    let mut cards_count: Vec<u32> = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let hits = card.number_of_hits();
        if hits > 0 {
            let card_count = cards_count[index];
            cards_count[(index + 1)..(index + hits + 1)]
                .iter_mut()
                .for_each(|count| *count += card_count);
        }
    }
    cards_count.iter().sum()
}

struct Card<'a> {
    wining_numbers: Vec<&'a str>,
    my_numbers: Vec<&'a str>,
}

impl<'a> Card<'a> {
    fn number_of_hits(&self) -> usize {
        self.wining_numbers
            .iter()
            .filter(|&number| self.my_numbers.contains(number))
            .count()
    }
}

fn parse_cards(input: &'_ str) -> Vec<Card<'_>> {
    input.lines().map(parse_card).collect()
}

fn parse_card(line: &'_ str) -> Card<'_> {
    let numbers = line.split_once(':').expect("Invalid card line").1;
    let (wining_numbers, my_numbers) = numbers.split_once('|').expect("Invalid card line");
    Card {
        wining_numbers: wining_numbers
            .trim()
            .split(' ')
            .filter(|v| !v.trim().is_empty())
            .collect(),
        my_numbers: my_numbers
            .trim()
            .split(' ')
            .filter(|v| !v.trim().is_empty())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn part_1_extra_checks() {
        let input = r"Card   1: 66 90 67 76 55 13 91 31 95  4 | 82 98 69  8 15  2 32 24 99 56 46 65 60 72 58 68 54 22 26  5 74 25 84 73 61
        Card 100: 22 39 65 11 89 20 95 35 53  5 | 83 40 91 76  8  7 74 67 86 21 12 48 15  3 50 10 44 55 13 88 45 94 81 19 70
        Card 215: 33  3 95 82 18 59 74  8 40 62 | 80 28 78 57 81 87 53 86 51 91 32 11 10 99 97 39  1 36  4 14 22 68 21 55 92";
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day4.txt");
        assert_eq!(part_1(input), 27845);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_2(input), 30);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day4.txt");
        assert_eq!(part_2(input), 9496801);
    }
}
