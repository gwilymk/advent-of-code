use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!(
        "Part 1: {}",
        input.lines().map(scratch_card_value).sum::<u32>()
    );
    println!("Part 2: {}", part2(&input));
}

fn part2(input: &str) -> u32 {
    let cards_and_number_of_wins = input
        .lines()
        .map(number_of_winning_numbers)
        .collect::<Vec<_>>();

    let mut number_of_each_card = vec![1; cards_and_number_of_wins.len()];

    for i in 0..cards_and_number_of_wins.len() {
        let number_of_wins = cards_and_number_of_wins[i];
        let number_of_this_card = number_of_each_card[i];

        for j in i..(i + number_of_wins as usize) {
            if let Some(cards) = number_of_each_card.get_mut(j + 1) {
                *cards += number_of_this_card;
            }
        }
    }

    number_of_each_card.iter().sum::<u32>()
}

fn scratch_card_value(input: &str) -> u32 {
    let number_of_winning = number_of_winning_numbers(input);
    if number_of_winning == 0 {
        0
    } else {
        2u32.pow(number_of_winning - 1)
    }
}

fn number_of_winning_numbers(input: &str) -> u32 {
    let (_, numbers) = input.split_once(':').unwrap();
    let (winning, draw) = numbers.split_once('|').unwrap();

    let winning: HashSet<u32> = winning
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let number_of_winning = draw
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .filter(|n| winning.contains(n))
        .count();

    number_of_winning as u32
}

#[test]
fn test_input_line_1() {
    let value = scratch_card_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(value, 8);
}

#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let value = part2(input);
    assert_eq!(value, 30);
}
