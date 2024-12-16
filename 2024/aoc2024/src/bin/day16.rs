use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(16);
    println!("Part 1: {}", reindeer_race(&input));
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

fn reindeer_race(input: &str) -> u32 {
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

    #[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
    struct Node {
        distance: u32,
        point: (i32, i32),
        direction: Direction,
    }

    type PosDir = ((i32, i32), Direction);

    let mut previous: HashMap<PosDir, PosDir> = HashMap::new();
    let mut q = BinaryHeap::new();
    let mut distance: HashMap<PosDir, u32> = HashMap::new();

    q.push(Reverse(Node {
        distance: 0,
        point: (start.x, start.y),
        direction: Direction::East,
    }));

    while let Some(Reverse(minimum)) = q.pop() {
        if minimum.point == (end.x, end.y) {
            return minimum.distance;
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
            if neighbour.distance
                < *distance
                    .get(&(neighbour.point, neighbour.direction))
                    .unwrap_or(&u32::MAX)
            {
                previous.insert(
                    (minimum.point, minimum.direction),
                    (neighbour.point, neighbour.direction),
                );
                distance.insert((neighbour.point, neighbour.direction), neighbour.distance);

                q.push(Reverse(neighbour.clone()));
            }
        }
    }

    panic!("Could not find route");
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

    assert_eq!(reindeer_race(input), 7036);
}
