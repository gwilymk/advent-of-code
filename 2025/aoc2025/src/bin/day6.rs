fn main() {
    let input = aoc2025::get_input(6);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let full_input: Vec<_> = input.lines().collect();

    let numbers_matrix: Vec<Vec<_>> = full_input[..full_input.len() - 1]
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let operations = full_input.last().unwrap().split_ascii_whitespace();

    operations
        .enumerate()
        .map(|(i, op)| {
            let numbers = numbers_matrix.iter().map(|row| row[i]);
            match op {
                "*" => numbers.product::<u64>(),
                "+" => numbers.sum(),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let full_input: Vec<_> = input.lines().collect();

    let mut numbers_matrix = vec![];
    let mut i = full_input[0].len();

    let mut current_line = vec![];
    while i > 0 {
        i -= 1;

        let mut value = 0u64;
        for line in &full_input[..full_input.len() - 1] {
            let c = line.as_bytes()[i];
            if c.is_ascii_digit() {
                let digit = c - b'0';
                value = value * 10 + digit as u64;
            }
        }

        if value == 0 {
            current_line.reverse();
            numbers_matrix.push(current_line);
            current_line = vec![];
        } else {
            current_line.push(value);
        }
    }

    current_line.reverse();
    numbers_matrix.push(current_line);

    numbers_matrix.reverse();

    let operations = full_input.last().unwrap().split_ascii_whitespace();

    operations
        .zip(numbers_matrix)
        .map(|(op, numbers)| match op {
            "*" => numbers.iter().product::<u64>(),
            "+" => numbers.iter().sum(),
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 4277556);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 3263827);
}
