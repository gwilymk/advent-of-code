fn main() {
    let input = aoc2025::get_input(6);

    println!("Part 1: {}", part1(&input));
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

#[cfg(test)]
const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 4277556);
}
