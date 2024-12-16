use std::collections::HashSet;

use aoc2024::{get_input, Direction, Grid2, Vector2D};

fn main() {
    let input = Map::parse(&get_input(6));
    println!("Part 1: {}", positions(&input).len());
    println!("Part 2: {}", additional_obstruction_locations(&input));
}

#[derive(Clone)]
struct Map {
    obstructions: Grid2<bool>,
    position: Vector2D<i32>,
    direction: Direction,
}

impl Map {
    fn parse(input: &str) -> Self {
        let obstructions = Grid2::parse(input, |line| line.chars().map(|c| c == '#').collect());

        let start_point = input.find('^').unwrap();
        let start_point_x = start_point % (obstructions.width + 1);
        let start_point_y = start_point / (obstructions.width + 1);

        Self {
            obstructions,
            position: Vector2D {
                x: start_point_x as i32,
                y: start_point_y as i32,
            },
            direction: Direction::North,
        }
    }

    fn contains_point(&self, point: Vector2D<i32>) -> bool {
        (0..self.obstructions.width as i32).contains(&point.x)
            && (0..self.obstructions.height as i32).contains(&point.y)
    }

    fn is_obstructed(&self, point: Vector2D<i32>) -> bool {
        *self.obstructions.get::<i32>(point).unwrap_or(&false)
    }

    fn add_obstruction(&mut self, point: Vector2D<i32>) {
        self.obstructions.set::<i32>(point, true);
    }
}

fn does_route_loop(input: &Map) -> bool {
    let mut current_point = input.position;
    let mut current_direction = input.direction;

    let mut visited_points = HashSet::new();

    while input.contains_point(current_point) {
        if !visited_points.insert((current_point, current_direction)) {
            return true;
        }

        loop {
            let potential_new_point = current_point + current_direction.into();

            if !input.is_obstructed(potential_new_point) {
                current_point = potential_new_point;
                break;
            }

            current_direction = current_direction.rotate_right();
        }
    }

    false
}

fn positions(input: &Map) -> HashSet<Vector2D<i32>> {
    let mut current_point = input.position;
    let mut current_direction = input.direction;

    let mut visited_points = HashSet::new();

    while input.contains_point(current_point) {
        visited_points.insert(current_point);

        loop {
            let potential_new_point = current_point + current_direction.into();

            if !input.is_obstructed(potential_new_point) {
                current_point = potential_new_point;
                break;
            }

            current_direction = current_direction.rotate_right();
        }
    }

    visited_points
}

fn additional_obstruction_locations(input: &Map) -> usize {
    let positions_to_search = positions(input);

    positions_to_search
        .iter()
        .filter(|point| {
            let mut input = input.clone();
            input.add_obstruction(**point);

            does_route_loop(&input)
        })
        .count()
}

#[test]
fn given_input() {
    let input = Map::parse(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    );

    assert_eq!(positions(&input).len(), 41);

    assert_eq!(additional_obstruction_locations(&input), 6);
}
