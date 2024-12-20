use std::collections::HashMap;

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(20);
    println!("Part 1: {}", part1(&input, 100));
}

fn find_cheats(input: &str) -> HashMap<(Vector2D<i32>, Vector2D<i32>), usize> {
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

        for (&first_cheat_cost, first_cheat_point) in
            maze_costs.neighbours_with_points::<i32>(point, false)
        {
            if first_cheat_cost != usize::MAX {
                continue;
            }

            for (&second_cheat_cost, second_cheat_point) in
                maze_costs.neighbours_with_points::<i32>(first_cheat_point, false)
            {
                if second_cheat_cost == usize::MAX {
                    continue;
                }

                if second_cheat_cost > cheat_start_cost + 2 {
                    result.insert(
                        (first_cheat_point, second_cheat_point),
                        second_cheat_cost - cheat_start_cost - 2,
                    );
                }
            }
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
    let cheats = find_cheats(input);

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
}
