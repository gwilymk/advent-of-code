fn main() {
    let input = aoc2025::get_input(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let instructions = parse(input);

    let mut password = 0;
    let mut dial_position = 50;

    for instruction in instructions {
        dial_position = (dial_position + instruction).rem_euclid(100);
        if dial_position == 0 {
            password += 1;
        }
    }

    password
}

fn part2(input: &str) -> i32 {
    let instructions = parse(input);

    let mut password = 0;
    let mut dial_position = 50;

    for instruction in instructions {
        let started_on_zero = dial_position == 0;

        dial_position += instruction;

        if dial_position >= 100 {
            password += dial_position / 100;
        }

        if !started_on_zero && dial_position < 0 {
            password += dial_position / -100 + 1;
        }

        if started_on_zero && dial_position <= -100 {
            password += dial_position / -100;
        }

        if dial_position == 0 {
            password += 1;
        }

        dial_position = dial_position.rem_euclid(100);
    }

    password
}

fn parse(input: &str) -> impl Iterator<Item = i32> {
    input.split('\n').map(|i| {
        let (dir, amount) = i.split_at(1);
        if dir == "L" {
            -amount.parse::<i32>().unwrap()
        } else {
            amount.parse::<i32>().unwrap()
        }
    })
}

#[cfg(test)]
const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[test]
fn example_part1() {
    assert_eq!(part1(TEST_INPUT), 3);
}

#[test]
fn example_part2() {
    assert_eq!(part2(TEST_INPUT), 6);
}
