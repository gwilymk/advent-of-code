use std::collections::HashMap;

fn main() {
    let input = aoc2024::get_input(1);

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .split('\n')
        .map(|pair| {
            let mut result = pair
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap());
            (result.next().unwrap(), result.next().unwrap())
        })
        .unzip()
}

fn part1(input: &str) -> usize {
    let mut lists: (Vec<i64>, Vec<i64>) = parse(input);

    lists.0.sort_by_key(|value| *value);
    lists.1.sort_by_key(|value| *value);

    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(v1, v2)| v1.abs_diff(*v2) as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    let lists: (Vec<i64>, Vec<i64>) = parse(input);

    let mut counts = HashMap::new();
    for value in lists.1 {
        counts.entry(value).and_modify(|v| *v += 1).or_insert(1);
    }

    lists
        .0
        .iter()
        .map(|v| counts.get(v).copied().unwrap_or(0) * *v)
        .sum::<i64>() as usize
}

#[test]
fn given_input() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(part1(input), 11);

    assert_eq!(part2(input), 31);
}
