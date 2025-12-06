use std::ops::RangeInclusive;

fn main() {
    let input = aoc2025::get_input(5);
    let (fresh_ranges, ingredients) = parse(&input);

    println!("Part 1: {}", part1(&fresh_ranges, &ingredients));
    println!("Part 2: {}", part2(&fresh_ranges));
}

fn part1(fresh_ranges: &[RangeInclusive<u64>], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| fresh_ranges.iter().any(|r| r.contains(ingredient)))
        .count()
}

fn part2(fresh_ranges: &[RangeInclusive<u64>]) -> u64 {
    let mut fresh_ranges = Vec::from(fresh_ranges);
    fresh_ranges.sort_unstable_by_key(|r| *r.start());

    // we want to deduplicate the ranges and combine overlapping ones
    // with the ranges sorted by start point, we can consider them in pairs
    let mut current = 0;
    while current < fresh_ranges.len() - 1 {
        let first = &fresh_ranges[current];
        let second = &fresh_ranges[current + 1];

        if first.end() >= second.start() {
            // the two ranges overlap
            let combined = *first.start()..=*second.end().max(first.end());

            fresh_ranges[current] = combined;
            fresh_ranges.remove(current + 1);
        } else {
            current += 1;
        }
    }

    // now we can count the number of elements in each range
    fresh_ranges
        .iter()
        .map(|range| *range.end() - *range.start() + 1)
        .sum()
}

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    (fresh_ranges, ingredients)
}

#[cfg(test)]
const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn test_part2() {
    let (ranges, _) = parse(TEST_INPUT);
    assert_eq!(part2(&ranges), 14);
}

#[test]
fn test_part2_manual_cases() {
    assert_eq!(part2(&[10..=10, 11..=13, 10..=10, 13..=14]), 5);
}
