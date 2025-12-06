fn main() {
    let input = aoc2025::get_input(5);

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
    let fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect();

    ingredients
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|ingredient| fresh_ranges.iter().any(|r| r.contains(ingredient)))
        .count()
}
