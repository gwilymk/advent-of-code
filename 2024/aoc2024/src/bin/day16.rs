use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(16);
    let (part1, part2) = reindeer_race(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    // 582 too high, 523 too low
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn neighbours(self) -> [Self; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }

    fn all() -> [Direction; 4] {
        use Direction::*;
        [North, East, South, West]
    }
}

impl From<&Direction> for Vector2D<i32> {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
        .into()
    }
}

fn reindeer_race(input: &str) -> (u32, usize) {
    let map = Grid2::parse(input, |line| line.chars().map(|c| c == '#').collect());
    let end = input.chars().position(|c| c == 'E').unwrap();
    let start = input.chars().position(|c| c == 'S').unwrap();

    let end = Vector2D::new(
        (end % (map.width + 1)) as i32,
        (end / (map.width + 1)) as i32,
    );
    let start = Vector2D::new(
        (start % (map.width + 1)) as i32,
        (start / (map.width + 1)) as i32,
    );

    #[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
    struct Node {
        distance: u32,
        point: (i32, i32),
        direction: Direction,
    }

    type PosDir = ((i32, i32), Direction);

    let mut q = BinaryHeap::new();
    let mut distance: HashMap<PosDir, u32> = HashMap::new();

    q.push(Reverse(Node {
        distance: 0,
        point: (start.x, start.y),
        direction: Direction::East,
    }));

    let mut end_pos_dir = None;

    while let Some(Reverse(minimum)) = q.pop() {
        if minimum.point == (end.x, end.y) && end_pos_dir.is_none() {
            end_pos_dir = Some((minimum.point, minimum.direction));
        }

        let neighbours = {
            let directional_neighbours = minimum.direction.neighbours();
            let step = Vector2D::from(&minimum.direction);
            [
                Node {
                    distance: minimum.distance + 1000,
                    point: minimum.point,
                    direction: directional_neighbours[0],
                },
                Node {
                    distance: minimum.distance + 1000,
                    point: minimum.point,
                    direction: directional_neighbours[1],
                },
                Node {
                    distance: minimum.distance + 1,
                    point: (minimum.point.0 + step.x, minimum.point.1 + step.y),
                    direction: minimum.direction,
                },
            ]
        };

        let neighbours = neighbours
            .iter()
            .filter(|n| map.get::<i32>(n.point) == Some(&false));

        for neighbour in neighbours {
            let current_distance = *distance
                .get(&(neighbour.point, neighbour.direction))
                .unwrap_or(&u32::MAX);

            if neighbour.distance <= current_distance {
                distance.insert((neighbour.point, neighbour.direction), neighbour.distance);

                if neighbour.distance != current_distance {
                    q.push(Reverse(neighbour.clone()));
                }
            }
        }
    }

    let end_distance = *Direction::all()
        .iter()
        .filter_map(|d| distance.get(&((end.x, end.y), *d)))
        .min()
        .unwrap();

    let mut best_seats = HashSet::new();
    best_seats.insert((start.x, start.y));

    fn collect_best_seats(
        point: PosDir,
        start: Vector2D<i32>,
        distance: &HashMap<((i32, i32), Direction), u32>,
        seats: &mut HashSet<(i32, i32)>,
    ) {
        seats.insert(point.0);

        if point.0 == (start.x, start.y) {
            return;
        }

        let current_cost = distance[&point];

        let neighbours = {
            let directional_neighbours = point.1.neighbours();
            let step: Vector2D<i32> = Vector2D::from(&point.1);
            [
                Node {
                    distance: current_cost.saturating_sub(1000),
                    point: point.0,
                    direction: directional_neighbours[0],
                },
                Node {
                    distance: current_cost.saturating_sub(1000),
                    point: point.0,
                    direction: directional_neighbours[1],
                },
                Node {
                    distance: current_cost - 1,
                    point: (point.0 .0 - step.x, point.0 .1 - step.y),
                    direction: point.1,
                },
            ]
        };

        for previous_point in neighbours {
            let neighbour_point = (previous_point.point, previous_point.direction);
            let Some(this_distance) = distance.get(&neighbour_point) else {
                continue;
            };

            if *this_distance == previous_point.distance {
                collect_best_seats(neighbour_point, start, distance, seats);
            }
        }
    }

    collect_best_seats(end_pos_dir.unwrap(), start, &distance, &mut best_seats);

    (end_distance, best_seats.len())
}

#[test]
fn given_input() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    assert_eq!(reindeer_race(input), (7036, 45));
}

#[test]
fn given_input2() {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    assert_eq!(reindeer_race(input), (11048, 64));
}
