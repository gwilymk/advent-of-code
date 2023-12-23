use std::{collections::HashSet, fmt::Debug};

fn main() {
    let forest = Forest::parse(include_str!("../input.txt"));
    println!(
        "Part 1: {}",
        forest
            .longest_walk(Point(1, 0), &HashSet::new(), true)
            .unwrap()
    );

    println!(
        "Part 2: {}",
        forest
            .longest_walk(Point(1, 0), &HashSet::new(), false)
            .unwrap()
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    SteepSlop(Direction),
}

struct Forest {
    tiles: Vec<Vec<Tile>>,
}

impl Forest {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Forest,
                        '.' => Tile::Path,
                        '>' => Tile::SteepSlop(Direction::Right),
                        '<' => Tile::SteepSlop(Direction::Left),
                        'v' => Tile::SteepSlop(Direction::Down),
                        '^' => Tile::SteepSlop(Direction::Up),
                        _ => panic!("Unknown character {c}"),
                    })
                    .collect()
            })
            .collect();

        Self { tiles }
    }

    fn longest_walk(
        &self,
        start_point: Point,
        visited: &HashSet<Point>,
        slippy_slopes: bool,
    ) -> Option<usize> {
        let mut visited = visited.clone();

        let mut current_point = start_point;
        let mut amount_to_add = 0;

        loop {
            let mut points_to_check = Vec::with_capacity(2);
            visited.insert(current_point);
            amount_to_add += 1;

            for direction in Direction::all() {
                let Some(new_point) = current_point.move_in_direction(direction) else {
                    continue;
                };

                if visited.contains(&new_point) {
                    continue;
                }

                let tile = self.tiles[new_point.1][new_point.0];
                if tile == Tile::Forest {
                    continue;
                }

                if let Tile::SteepSlop(slope_direction) = tile {
                    if slope_direction != direction && slippy_slopes {
                        continue;
                    }
                }

                if new_point.1 == self.tiles.len() - 1 {
                    // this is the edge of the map!
                    return Some(amount_to_add);
                }

                points_to_check.push(new_point);
            }

            if points_to_check.is_empty() {
                return None; // dead end
            }

            if points_to_check.len() > 1 {
                return points_to_check
                    .iter()
                    .filter_map(|&new_start_point| {
                        self.longest_walk(new_start_point, &visited, slippy_slopes)
                    })
                    .max()
                    .map(|value| value + amount_to_add);
            }

            current_point = points_to_check[0];
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Point {
    fn move_in_direction(self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Point(self.0, self.1.checked_sub(1)?),
            Direction::Right => Point(self.0 + 1, self.1),
            Direction::Down => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0.checked_sub(1)?, self.1),
        })
    }
}

#[test]
fn given_input() {
    let forest = Forest::parse(
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
    );

    assert_eq!(
        forest.longest_walk(Point(1, 0), &HashSet::new(), true),
        Some(94)
    );

    assert_eq!(
        forest.longest_walk(Point(1, 0), &HashSet::new(), false),
        Some(154)
    );
}
