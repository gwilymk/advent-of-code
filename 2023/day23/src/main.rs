use std::{collections::HashSet, fmt::Debug};

fn main() {
    let forest = Forest::parse(include_str!("../input.txt"));
    println!("Part 2: {}", forest.longest_walk(false));
    println!("Part 1: {}", forest.longest_walk(true));
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

    fn longest_walk(&self, slippy_slopes: bool) -> usize {
        let mut graph = petgraph::graphmap::UnGraphMap::new();

        let start_point = Point(1, 0);
        let end_point = Point(self.tiles[0].len() - 2, self.tiles.len() - 1);

        self.build_graph(&mut graph, start_point, slippy_slopes);
        // let graph = petgraph::algo::condensation(graph.into_graph::<usize>(), false);

        println!("{:?}", petgraph::dot::Dot::new(&graph));

        let paths =
            petgraph::algo::simple_paths::all_simple_paths(&graph, start_point, end_point, 0, None);

        let mut max = 0;
        for length in paths.map(|path: Vec<_>| path.len() - 1) {
            max = length.max(max);
        }

        max
    }

    fn build_graph(
        &self,
        graph: &mut petgraph::graphmap::UnGraphMap<Point, ()>,
        start_point: Point,
        _slippy_slopes: bool,
    ) {
        let mut nodes_to_search_from = vec![start_point];
        let mut searched_nodes = HashSet::new();

        while let Some(current_point) = nodes_to_search_from.pop() {
            searched_nodes.insert(current_point);

            for direction in Direction::all() {
                let Some(point_to_search) = current_point.move_in_direction(direction) else {
                    continue;
                };

                if point_to_search.1 == self.tiles.len() {
                    // coming off the edge of the map
                    continue;
                }

                let tile = self.tiles[point_to_search.1][point_to_search.0];

                if tile != Tile::Forest && !searched_nodes.contains(&point_to_search) {
                    nodes_to_search_from.push(point_to_search);
                    graph.add_edge(current_point, point_to_search, ());
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    assert_eq!(forest.longest_walk(false), 154);
    // assert_eq!(forest.longest_walk(true), 94);
}
