use aoc2025::{Grid2, Vector2D};

fn main() {
    let input = aoc2025::get_input(4);
    let input = parse(&input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
}

fn parse(input: &str) -> Grid2<bool> {
    Grid2::parse(input, |line| line.chars().map(|c| c == '@').collect())
}

fn part1(input: &Grid2<bool>) -> usize {
    accessible(input).count()
}

fn part2(mut input: Grid2<bool>) -> usize {
    let mut count = 0;
    loop {
        let removed = remove_accessible(&mut input);
        count += removed;

        if removed == 0 {
            return count;
        }
    }
}

// returns the number of items which were removed
fn remove_accessible(input: &mut Grid2<bool>) -> usize {
    let accessible = accessible(input).collect::<Vec<_>>();
    let count = accessible.len();
    for point in accessible {
        input.set::<i32>(point, false);
    }

    count
}

fn accessible(input: &Grid2<bool>) -> impl Iterator<Item = Vector2D<i32>> {
    input
        .iter()
        .filter(|(_, has)| **has)
        .map(|(pos, _)| pos)
        .filter(|pos| {
            input
                .neighbours::<i32>(*pos, true)
                .filter(|has| **has)
                .count()
                < 4
        })
}
