fn main() {
    let input = aoc2025::get_input(2);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    parse(input)
        .map(|(start, end)| invalid_in_range(start, end))
        .sum()
}

fn part2(input: &str) -> u64 {
    parse(input)
        .map(|(start, end)| invalid_in_range2(start, end))
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|r| {
        let (first, second) = r.split_once('-').unwrap();
        (
            first.parse::<u64>().unwrap(),
            second.parse::<u64>().unwrap(),
        )
    })
}

fn invalid_in_range(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    for value in start..=end {
        // get the number of digits in value
        let digits = value.checked_ilog10().unwrap_or(0) + 1;
        if !digits.is_multiple_of(2) {
            continue;
        }

        // it's a repeat if it's a multiple of 10001 (for some number of 0s)
        let check = 10_u64.pow(digits / 2) + 1;

        if value.is_multiple_of(check) {
            sum += value;
        }
    }

    sum
}

fn invalid_in_range2(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    'next_range: for value in start..=end {
        let value_str = value.to_string();
        'outer: for i in 1..=(value_str.len() / 2) {
            if !value_str.len().is_multiple_of(i) {
                continue;
            }

            let test = &value_str[0..i];
            for j in 1..(value_str.len() / i) {
                if &value_str[j * i..(j + 1) * i] != test {
                    continue 'outer;
                }
            }

            sum += value;
            continue 'next_range;
        }
    }

    sum
}

#[cfg(test)]
const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1227775554);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 4174379265);
}
