use std::collections::HashMap;

fn main() {
    let input = aoc2024::get_input(11);

    println!("Part 1: {}", blinks(&input, 25));
    println!("Part 2: {}", blinks(&input, 75));
}

fn blinks(input: &str, blinks: usize) -> usize {
    let stones = input.split(' ').map(|i| i.parse::<usize>().unwrap());

    let mut cache = HashMap::new();
    stones
        .map(|stone| number_after_iterations(stone, blinks, &mut cache))
        .sum()
}

fn number_after_iterations(
    stone: usize,
    iterations_remaining: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if iterations_remaining == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(stone, iterations_remaining)) {
        return *cached;
    }

    if stone == 0 {
        let result = number_after_iterations(1, iterations_remaining - 1, cache);
        cache.insert((stone, iterations_remaining), result);
        return result;
    }

    let as_str = stone.to_string();
    if as_str.len() % 2 == 0 {
        let (first_half, second_half) = as_str.split_at(as_str.len() / 2);

        let first =
            number_after_iterations(first_half.parse().unwrap(), iterations_remaining - 1, cache);
        let second = number_after_iterations(
            second_half.parse().unwrap(),
            iterations_remaining - 1,
            cache,
        );

        let result = first + second;
        cache.insert((stone, iterations_remaining), result);
        return result;
    }

    let result = number_after_iterations(stone * 2024, iterations_remaining - 1, cache);
    cache.insert((stone, iterations_remaining), result);

    result
}

#[test]
fn given_input() {
    assert_eq!(blinks("125 17", 25), 55312);
}
