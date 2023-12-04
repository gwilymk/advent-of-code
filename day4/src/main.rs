use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!(
        "Part 1: {}",
        input.lines().map(scratch_card_value).sum::<u32>()
    )
}

fn scratch_card_value(input: &str) -> u32 {
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

    if number_of_winning == 0 {
        0
    } else {
        2u32.pow(number_of_winning as u32 - 1)
    }
}

#[test]
fn test_input_line_1() {
    let value = scratch_card_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(value, 8);
}
