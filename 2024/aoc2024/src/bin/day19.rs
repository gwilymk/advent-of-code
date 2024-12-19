use aoc2024::get_input;
use regex::Regex;

fn main() {
    let input = get_input(19);
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let (towel_availablilities, desired_patterns) = input.split_once("\n\n").unwrap();
    let towel_availabilities_regex = towel_availablilities.replace(", ", "|");

    let towel_regex = Regex::new(&format!("^(:?{})+$", towel_availabilities_regex)).unwrap();

    desired_patterns
        .split('\n')
        .filter(|p| towel_regex.is_match(p))
        .count()
}

#[test]
fn given_input() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part1(input), 6);
}
