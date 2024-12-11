use std::collections::HashSet;

fn main() {
    let puzzle_input = aoc2024::get_input(5);
    let input = Input::parse(&puzzle_input);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

struct Input {
    orderings: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let (orderings, updates) = input.split_once("\n\n").unwrap();

        let orderings = orderings
            .split('\n')
            .map(|o| {
                let (before, after) = o.split_once('|').unwrap();
                (
                    before.parse::<usize>().unwrap(),
                    after.parse::<usize>().unwrap(),
                )
            })
            .collect::<HashSet<_>>();

        let updates = updates
            .split('\n')
            .map(|update| {
                update
                    .split(',')
                    .map(|u| u.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { orderings, updates }
    }
}

fn part1(input: &Input) -> usize {
    valid_updates(input)
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .updates
        .iter()
        .filter(|update| !update_is_in_order(update, &input.orderings))
        .map(|incorrect| {
            let mut working = incorrect.to_vec();

            while !update_is_in_order(&working, &input.orderings) {
                for i in 0..incorrect.len() {
                    for j in i + 1..incorrect.len() {
                        if input.orderings.contains(&(working[j], working[i])) {
                            working.swap(i, j);
                        }
                    }
                }
            }

            working[working.len() / 2]
        })
        .sum()
}

fn valid_updates(input: &Input) -> impl Iterator<Item = &'_ Vec<usize>> {
    input
        .updates
        .iter()
        .filter(|update| update_is_in_order(update, &input.orderings))
}

fn update_is_in_order(update: &[usize], ordering: &HashSet<(usize, usize)>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if ordering.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }

    true
}

#[test]
fn given_input() {
    let input = Input::parse(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );

    assert_eq!(part1(&input), 143);
    assert_eq!(part2(&input), 123);
}
