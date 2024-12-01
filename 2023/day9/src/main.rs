fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect::<Vec<_>>();
            extrapolate_end(&numbers)
        })
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect::<Vec<_>>();
            extrapolate_start(&numbers)
        })
        .sum::<i64>()
}

fn extrapolate_end(input: &[i64]) -> i64 {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }

    let differences: Vec<_> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    extrapolate_end(&differences) + input.last().unwrap()
}

fn extrapolate_start(input: &[i64]) -> i64 {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }

    let differences: Vec<_> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    input[0] - extrapolate_start(&differences)
}

#[test]
fn test_extrapolate() {
    assert_eq!(extrapolate_end(&[0, 3, 6, 9, 12, 15]), 18);
    assert_eq!(extrapolate_end(&[1, 3, 6, 10, 15, 21]), 28);
    assert_eq!(extrapolate_end(&[10, 13, 16, 21, 30, 45]), 68);
}
