use aoc2024::get_input;

fn main() {
    let input = get_input(22);
    println!("Part 1: {}", part1(&input));
}

struct Rng {
    secret: usize,
}

impl Iterator for Rng {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.secret = ((self.secret * 64) ^ self.secret) % 16777216;
        self.secret = ((self.secret / 32) ^ self.secret) % 16777216;
        self.secret = ((self.secret * 2048) ^ self.secret) % 16777216;

        Some(self.secret)
    }
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            let mut rng = Rng {
                secret: line.parse().unwrap(),
            };

            rng.nth(1999).unwrap()
        })
        .sum()
}

#[test]
fn case123() {
    let rng = Rng { secret: 123 };
    let sequence = [
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];

    let first_10 = rng.take(10).collect::<Vec<_>>();

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
