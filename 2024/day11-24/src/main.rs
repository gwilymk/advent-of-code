fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
}

fn part1(input: &str) -> usize {
    let mut stones = input
        .split(' ')
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut new_stones = vec![];

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            let as_str = stone.to_string();
            if as_str.len() % 2 == 0 {
                let (first_half, second_half) = as_str.split_at(as_str.len() / 2);
                new_stones.push(first_half.parse().unwrap());
                new_stones.push(second_half.parse().unwrap());
                continue;
            }

            new_stones.push(stone * 2024);
        }

        stones = new_stones;
    }

    stones.len()
}

#[test]
fn given_input() {
    assert_eq!(part1("125 17"), 55312);
}
