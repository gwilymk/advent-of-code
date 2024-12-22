use std::collections::HashMap;

use aoc2024::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(22);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Rng {
    secret: usize,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Sequence {
    changes: [i32; 4],
}

impl Iterator for Rng {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let old_secret = self.secret;

        self.secret = ((self.secret * 64) ^ self.secret) % 16777216;
        self.secret = ((self.secret / 32) ^ self.secret) % 16777216;
        self.secret = ((self.secret * 2048) ^ self.secret) % 16777216;

        Some(old_secret)
    }
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            let mut rng = Rng {
                secret: line.parse().unwrap(),
            };

            rng.nth(2000).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut sequences: HashMap<Sequence, usize> = HashMap::new();

    for line in input.split('\n') {
        let rng = Rng {
            secret: line.parse().unwrap(),
        };

        let mut this_winnings = HashMap::<Sequence, usize>::new();
        for (a, b, c, d, e) in rng.take(2001).tuple_windows() {
            fn get_change(a: usize, b: usize) -> i32 {
                ((a % 10) as i32) - ((b % 10) as i32)
            }

            let sequence = Sequence {
                changes: [
                    get_change(b, a),
                    get_change(c, b),
                    get_change(d, c),
                    get_change(e, d),
                ],
            };

            this_winnings.entry(sequence).or_insert(e % 10);
        }

        for (winning, amount) in this_winnings {
            *sequences.entry(winning).or_default() += amount;
        }
    }

    *sequences.values().max().unwrap()
}

#[test]
fn case123() {
    let rng = Rng { secret: 123 };
    let sequence = [
        123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
        5908254,
    ];

    let first_10 = rng.take(11).collect::<Vec<_>>();

    assert_eq!(first_10, sequence);
}

#[test]
fn given_input() {
    let input = "1
10
100
2024";

    assert_eq!(part1(input), 37327623);
}

#[test]
fn given_input2() {
    let input = "1
2
3
2024";

    assert_eq!(part2(input), 23);
}
