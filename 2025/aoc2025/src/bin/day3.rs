fn main() {
    let input = aoc2025::get_input(3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    input.split('\n').map(|line| max_joltage(line, 2)).sum()
}

fn part2(input: &str) -> u64 {
    input.split('\n').map(|line| max_joltage(line, 12)).sum()
}

fn max_joltage(line: &str, max_batteries: usize) -> u64 {
    let mut max_index = 0;
    let parsed: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut joltage = 0;

    for i in 0..max_batteries {
        let next_value = *parsed
            .iter()
            .take(parsed.len() - (max_batteries - i) + 1)
            .skip(max_index)
            .max()
            .unwrap();

        let next_index = parsed
            .iter()
            .skip(max_index)
            .position(|d| d == &next_value)
            .unwrap();

        joltage = joltage * 10 + u64::from(next_value);
        max_index += next_index + 1;
    }

    joltage
}

#[test]
fn test_part1() {
    assert_eq!(max_joltage("987654321111111", 2), 98);
    assert_eq!(max_joltage("811111111111119", 2), 89);
    assert_eq!(max_joltage("234234234234278", 2), 78);
    assert_eq!(max_joltage("818181911112111", 2), 92);
    assert_eq!(
        max_joltage(
            "2232212212212222211221231124224222213132222133122224222123222112324122222122221322222225222342243112",
            2
        ),
        54
    );
}

#[test]
fn test_part2() {
    assert_eq!(max_joltage("987654321111111", 12), 987654321111);
    assert_eq!(max_joltage("811111111111119", 12), 811111111119);
    assert_eq!(max_joltage("234234234234278", 12), 434234234278);
    assert_eq!(max_joltage("818181911112111", 12), 888911112111);
}
