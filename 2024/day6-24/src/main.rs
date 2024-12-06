use std::collections::HashSet;

fn main() {
    let input = Map::parse(include_str!("./input.txt"));
    println!("Part 1: {}", positions(&input).len());
    println!("Part 2: {}", additional_obstruction_locations(&input));
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn in_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(self: Direction) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Map {
    obstructions: Vec<Vec<bool>>,
    position: Point,
    direction: Direction,
}

impl Map {
    fn parse(input: &str) -> Self {
        let obstructions = input
            .split('\n')
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let start_point = input.find('^').unwrap();
        let start_point_x = start_point % (obstructions[0].len() + 1);
        let start_point_y = start_point / (obstructions[0].len() + 1);

        Self {
            obstructions,
            position: Point {
                x: start_point_x as isize,
                y: start_point_y as isize,
            },
            direction: Direction::Up,
        }
    }

    fn contains_point(&self, point: Point) -> bool {
        if point.x < 0 || point.x >= self.obstructions[0].len() as isize {
            return false;
        }

        if point.y < 0 || point.y >= self.obstructions.len() as isize {
            return false;
        }

        true
    }

    fn is_obstructed(&self, point: Point) -> bool {
        if !self.contains_point(point) {
            return false;
        }

        self.obstructions[point.y as usize][point.x as usize]
    }

    fn add_obstruction(&mut self, point: Point) {
        self.obstructions[point.y as usize][point.x as usize] = true;
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
            let potential_new_point = current_point.in_direction(current_direction);

            if !input.is_obstructed(potential_new_point) {
                current_point = potential_new_point;
                break;
            }

            current_direction = current_direction.rotate_right();
        }
    }

    false
}

fn positions(input: &Map) -> HashSet<Point> {
    let mut current_point = input.position;
    let mut current_direction = input.direction;

    let mut visited_points = HashSet::new();

    while input.contains_point(current_point) {
        visited_points.insert(current_point);

        loop {
            let potential_new_point = current_point.in_direction(current_direction);

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
