use std::collections::HashMap;

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(20);
    println!("Part 1: {}", part1(&input, 100));
    println!("Part 2: {}", part2(&input, 100));
}

fn find_cheats(
    input: &str,
    max_length: i32,
    part_2_tracking: bool,
) -> HashMap<(Vector2D<i32>, Vector2D<i32>), usize> {
    let maze = Grid2::parse(input, |line| line.chars().map(|c| c == '#').collect());
    let start = input.find('S').unwrap() as i32;
    let end = input.find('E').unwrap() as i32;

    let start_pos = Vector2D::new(
        start % (maze.width as i32 + 1),
        start / (maze.width as i32 + 1),
    );
    let end_pos = Vector2D::new(end % (maze.width as i32 + 1), end / (maze.width as i32 + 1));

    let maze_costs: Grid2<usize> = calculate_costs(&maze, start_pos, end_pos);

    let mut result = HashMap::new();

    for (point, &cheat_start_cost) in maze_costs.iter() {
        if cheat_start_cost == usize::MAX {
            continue;
        }

        for x_diff in -max_length..=max_length {
            for y_diff in -max_length..=max_length {
                let diff = Vector2D::new(x_diff, y_diff);

                if diff.manhattan_distance() > max_length {
                    continue;
                }

                let second_cheat_point = point + diff;
                let Some(&second_cheat_cost) = maze_costs.get::<i32>(second_cheat_point) else {
                    continue;
                };

                if second_cheat_cost == usize::MAX {
                    continue;
                }

                if second_cheat_cost > cheat_start_cost + diff.manhattan_distance() as usize {
                    result.insert(
                        (
                            if part_2_tracking {
                                point
                            } else {
                                (point + second_cheat_point) / 2
                            },
                            second_cheat_point,
                        ),
                        second_cheat_cost - cheat_start_cost - diff.manhattan_distance() as usize,
                    );
                }
            }
            // }
        }
    }

    result
}

fn calculate_costs(maze: &Grid2<bool>, start: Vector2D<i32>, end: Vector2D<i32>) -> Grid2<usize> {
    let mut result = Grid2::new_with(maze.width, maze.height, || usize::MAX);

    result.set::<i32>(start, 0);

    let mut current = start;
    let mut current_cost = 0;
    while current != end {
        for (&is_wall, neighbour) in maze.neighbours_with_points::<i32>(current, false) {
            if is_wall {
                continue;
            }

            if *result.get::<i32>(neighbour).unwrap() == usize::MAX {
                result.set::<i32>(neighbour, current_cost + 1);
                current_cost += 1;
                current = neighbour;
                break;
            }
        }
    }

    result
}

fn part1(input: &str, threshold: usize) -> usize {
    let cheats = find_cheats(input, 2, false);

    cheats
        .iter()
        .filter(|(_, amount)| **amount >= threshold)
        .count()
}

fn part2(input: &str, threshold: usize) -> usize {
    let cheats = find_cheats(input, 20, true);

    let mut amounts: HashMap<usize, usize> = HashMap::new();
    for &amount in cheats.values() {
        if amount < threshold {
            continue;
        }

        *amounts.entry(amount).or_default() += 1;
    }
    let mut amounts: Vec<(usize, usize)> = amounts.into_iter().collect::<Vec<_>>();
    amounts.sort();
    println!("{amounts:#?}");

    cheats
        .iter()
        .filter(|(_, amount)| **amount >= threshold)
        .count()
}

#[test]
fn given_input() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    assert_eq!(part1(input, 20), 5);
    assert_eq!(
        part2(input, 50),
        32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    );
}
