use agb_fixnum::Vector2D;
use std::collections::{HashMap, HashSet};

fn main() {
    let map = Map::parse(include_str!("input.txt"));
    println!("Part 1: {}", map.antinode_locations().len());
    println!("Part 2: {}", map.antinode_locations2().len());
}

struct Map {
    nodes: HashMap<char, Vec<Vector2D<i32>>>,

    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut nodes: HashMap<char, Vec<Vector2D<i32>>> = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.split('\n').enumerate() {
            width = line.len();
            height = y + 1;

            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                nodes
                    .entry(c)
                    .or_default()
                    .push(Vector2D::new(x as i32, y as i32))
            }
        }

        Self {
            nodes,
            width,
            height,
        }
    }

    fn antinode_locations(&self) -> HashSet<Vector2D<i32>> {
        let mut antinode_locations = HashSet::new();

        for (_, locations) in &self.nodes {
            for j in 0..locations.len() {
                for i in (j + 1)..locations.len() {
                    let first = locations[i];
                    let second = locations[j];

                    let anti_node1 = first + (first - second);
                    let anti_node2 = second + second - first;

                    if self.is_in_bounds(anti_node1) {
                        antinode_locations.insert(anti_node1);
                    }

                    if self.is_in_bounds(anti_node2) {
                        antinode_locations.insert(anti_node2);
                    }
                }
            }
        }

        antinode_locations
    }

    fn antinode_locations2(&self) -> HashSet<Vector2D<i32>> {
        let mut antinode_locations = HashSet::new();

        for (_, locations) in &self.nodes {
            for j in 0..locations.len() {
                for i in (j + 1)..locations.len() {
                    let first = locations[i];
                    let second = locations[j];

                    let difference = second - first;

                    antinode_locations.insert(first);
                    antinode_locations.insert(second);

                    for i in 0.. {
                        let antinode1 = first + difference * i;
                        let antinode2 = first + difference * -i;
                        let mut either_in_bounds = false;

                        if self.is_in_bounds(antinode1) {
                            antinode_locations.insert(antinode1);
                            either_in_bounds = true;
                        }

                        if self.is_in_bounds(antinode2) {
                            antinode_locations.insert(antinode2);
                            either_in_bounds = true;
                        }

                        if !either_in_bounds {
                            break;
                        }
                    }
                }
            }
        }

        antinode_locations
    }

    fn is_in_bounds(&self, point: Vector2D<i32>) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }
}

#[test]
fn given_input() {
    let map = Map::parse(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    );

    assert_eq!(map.antinode_locations().len(), 14);
    assert_eq!(map.antinode_locations2().len(), 34);
}
