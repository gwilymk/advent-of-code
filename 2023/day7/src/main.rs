use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", total_score(input, false));
    println!("Part 2: {}", total_score(input, true));
}

fn total_score(input: &str, include_jokers: bool) -> usize {
    let mut input: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, score) = line.split_once(' ').unwrap();
            let hand = Hand::parse(hand, include_jokers).unwrap();
            let score: usize = score.parse().unwrap();

            (hand, score)
        })
        .collect();

    input.sort_by(|a, b| a.0.cmp(&b.0));

    input
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum::<usize>()
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    hand_type: HandType,
    cards: [usize; 5],
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        for (mine, yours) in self.cards.iter().zip(other.cards.iter()) {
            match mine.cmp(yours) {
                Ordering::Equal => {}
                ord => return Some(ord),
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn parse(input: &str, include_jokers: bool) -> Result<Self, String> {
        let mut cards = [0; 5];

        for (i, c) in input.chars().enumerate() {
            match c {
                '2'..='9' => cards[i] = c.to_digit(10).unwrap() as usize,
                'T' => cards[i] = 10,
                'J' => cards[i] = if include_jokers { 1 } else { 11 },
                'Q' => cards[i] = 12,
                'K' => cards[i] = 13,
                'A' => cards[i] = 14,
                _ => return Err(format!("Invalid letter {c}")),
            }
        }

        // the joker should turn into whichever card is the most common
        let joker_card = cards
            .iter()
            .filter(|&&card| card != 1)
            .map(|&card| (card, cards.iter().filter(|&&c| c == card).count()))
            .max_by_key(|(_, count)| *count)
            .unwrap_or((1, 1))
            .0;

        let mut mapped_cards = cards;
        for card in &mut mapped_cards {
            if *card == 1 {
                *card = joker_card;
            }
        }

        let distinct_cards = mapped_cards.iter().collect::<HashSet<_>>();
        let number_of_distinct_cards = distinct_cards.len();

        let hand_type = match number_of_distinct_cards {
            1 => HandType::FiveOfAKind,
            2 => {
                let first_card = mapped_cards[0];
                let number_of_first_card = mapped_cards
                    .iter()
                    .filter(|&&card| card == first_card)
                    .count();

                if number_of_first_card == 1 || number_of_first_card == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                let count_of_most_common_card = distinct_cards
                    .iter()
                    .map(|&&card| mapped_cards.iter().filter(|&&c| c == card).count())
                    .max()
                    .unwrap();

                if count_of_most_common_card == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Should never have more than 5 distinct cards"),
        };

        Ok(Hand { cards, hand_type })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[test]
fn check_if_hands_can_parse() {
    let hand: Hand = Hand::parse("32T3K", false).unwrap();
    assert_eq!(hand.hand_type, HandType::OnePair);
    assert_eq!(hand.cards, [3, 2, 10, 3, 13]);

    assert_eq!(
        Hand::parse("KK677", false).unwrap().hand_type,
        HandType::TwoPair
    );
    assert_eq!(
        Hand::parse("KTJJT", false).unwrap().hand_type,
        HandType::TwoPair
    );

    assert_eq!(
        Hand::parse("T55J5", false).unwrap().hand_type,
        HandType::ThreeOfAKind
    );
    assert_eq!(
        Hand::parse("QQQJA", false).unwrap().hand_type,
        HandType::ThreeOfAKind
    );
}

#[test]
fn given_input() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    assert_eq!(total_score(input, false), 6440);
    assert_eq!(total_score(input, true), 5905);
}
