use std::ops::RangeInclusive;

fn main() {
    let input = aoc2025::get_input(5);
    let (fresh_ranges, ingredients) = parse(&input);

    println!("Part 1: {}", part1(&fresh_ranges, &ingredients));
}

fn part1(fresh_ranges: &[RangeInclusive<u64>], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| fresh_ranges.iter().any(|r| r.contains(ingredient)))
        .count()
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
