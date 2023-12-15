fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum::<usize>()
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0, |acc, next| (acc + next as usize) * 17 % 256)
}
