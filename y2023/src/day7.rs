use std::collections::HashMap;

const JOKER_11: u8 = 11;
const JOKER_1: u8 = 1;

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    let mut ordered_hands = input
        .lines()
        .map(|hand| parse_hand(hand, JOKER_11))
        .collect::<Vec<Hand>>();
    ordered_hands.sort();
    ordered_hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u32 + 1))
        .sum()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    let mut ordered_hands = input
        .lines()
        .map(|hand| parse_hand(hand, JOKER_1))
        .collect::<Vec<Hand>>();
    ordered_hands.sort();
    ordered_hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u32 + 1))
        .sum()
}

fn parse_hand(hand: &str, joker_value: u8) -> Hand {
    let (cards, bid) = hand.split_once(' ').expect("Invalid input");
    let cards = parse_cards(cards, joker_value);
    let bid = bid.trim().parse().expect("Invalid bid");
    let hand_type = parse_type_of_hand(&cards, joker_value);
    Hand {
        cards,
        bid,
        hand_type,
    }
}

fn parse_cards(cards: &str, joker_value: u8) -> Vec<u8> {
    cards
        .chars()
        .map(|c| match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => joker_value,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid char {c}"),
        })
        .collect()
}
fn parse_type_of_hand(cards: &[u8], joker_value: u8) -> HandType {
    let mut cards_map = HashMap::new();
    for c in cards {
        *cards_map.entry(c).or_insert(0) += 1;
    }

    let mut hand_type = match cards_map.len() {
        1 => HandType::FiveOfKind,
        2 if cards_map.values().any(|&c| c == 4) => HandType::FourOfKind,
        2 => HandType::FullHouse,
        3 if cards_map.values().any(|&c| c == 3) => HandType::ThreeOfKind,
        3 => HandType::TwoPair,
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        x => panic!("Invalid cards size {x}"),
    };
    if joker_value == JOKER_1 {
        let number_of_jokers = cards_map.get(&JOKER_1).unwrap_or(&0);
        hand_type = match (hand_type, number_of_jokers) {
            (HandType::FourOfKind, 1) => HandType::FiveOfKind,
            (HandType::FourOfKind, 4) => HandType::FiveOfKind,
            (HandType::FullHouse, 3) => HandType::FiveOfKind,
            (HandType::FullHouse, 2) => HandType::FiveOfKind,
            (HandType::ThreeOfKind, 1) => HandType::FourOfKind,
            (HandType::ThreeOfKind, 3) => HandType::FourOfKind,
            (HandType::TwoPair, 2) => HandType::FourOfKind,
            (HandType::TwoPair, 1) => HandType::FullHouse,
            (HandType::OnePair, 1) => HandType::ThreeOfKind,
            (HandType::OnePair, 2) => HandType::ThreeOfKind,
            (HandType::HighCard, 1) => HandType::OnePair,
            (x, _) => x,
        }
    }

    hand_type
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
    hand_type: HandType,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{parse_hand, HandType, JOKER_1};

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_1(input), 6440);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day7.txt");
        assert_eq!(part_1(input), 250254244);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_2(input), 5905);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day7.txt");
        assert_eq!(part_2(input), 250087440);
    }

    #[test]
    fn part_2_extra_tests() {
        assert_eq!(
            parse_hand("JJJJJ 1", JOKER_1).hand_type,
            HandType::FiveOfKind
        );
        assert_eq!(
            parse_hand("687J7 1", JOKER_1).hand_type,
            HandType::ThreeOfKind
        );
        assert_eq!(
            parse_hand("KJKKJ 1", JOKER_1).hand_type,
            HandType::FiveOfKind
        );
        assert_eq!(
            parse_hand("KJKKJ 1", JOKER_1).hand_type,
            HandType::FiveOfKind
        );
    }

    #[test]
    fn part_2_extra_hands_tests() {
        assert_eq!(part_2("KJKKJ 1 \nJQK63 2 \n2QK63 3  "), 10);
        assert_eq!(part_2("9K9KK 1\n55553 4\nJAJJ9 3"), 1 + 3 * 2 + 4 * 3);
    }
}
