fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect::<Vec<_>>();
            extrapolate(&numbers)
        })
        .sum::<i64>()
}

fn extrapolate(input: &[i64]) -> i64 {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }

    let differences: Vec<_> = input
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    extrapolate(&differences) + input.last().unwrap()
}

#[test]
fn test_extrapolate() {
    assert_eq!(extrapolate(&[0, 3, 6, 9, 12, 15]), 18);
    assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]), 28);
    assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]), 68);
}
