use std::fmt::Debug;

use colored::Colorize;

fn main() {
    let city_map = CityMap::parse(include_str!("../input.txt"));
    println!("Part 1: {}", city_map.minimum_route_cost(1, 3));
    println!("Part 2: {}", city_map.minimum_route_cost(3, 10));
}

struct CityMap {
    heat_loss: Vec<Vec<usize>>,
}

impl CityMap {
    fn parse(input: &str) -> Self {
        let heat_loss = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as _).collect())
            .collect();

        Self { heat_loss }
    }

    fn minimum_route_cost(&self, minimum_distance: usize, maximum_distance: usize) -> usize {
        let mut graph = petgraph::graphmap::DiGraphMap::new();

        for y in 0..self.heat_loss.len() {
            for x in 0..self.heat_loss[0].len() {
                for direction in Direction::all() {
                    let mut heat_loss = self.heat_loss[y][x];

                    for distance in 1..minimum_distance {
                        let Some(target_point) = direction.move_point((x, y), distance) else {
                            continue;
                        };

                        if target_point.0 >= self.heat_loss[0].len()
                            || target_point.1 >= self.heat_loss.len()
                        {
                            continue;
                        }

                        heat_loss += self.heat_loss[target_point.1][target_point.0];
                    }

                    for distance in minimum_distance..=maximum_distance {
                        let Some(target_point) = direction.move_point((x, y), distance) else {
                            continue;
                        };

                        // if distance == maximum_distance && x == 0 && y == 0 {
                        //     continue;
                        // }

                        if target_point.0 >= self.heat_loss[0].len()
                            || target_point.1 >= self.heat_loss.len()
                        {
                            continue;
                        }

                        heat_loss += self.heat_loss[target_point.1][target_point.0];

                        let speedy_node = GraphNode::Speedy((x, y), direction, distance);

                        for other_direction in Direction::all_except(direction) {
                            if other_direction == direction.opposite() {
                                continue;
                            }

                            let Some(new_point) = other_direction.move_point(target_point, 1) else {
                                continue;
                            };

                            if new_point.0 >= self.heat_loss[0].len()
                                || new_point.1 >= self.heat_loss.len()
                            {
                                continue;
                            }

                            for valid_direction in Direction::all_except(other_direction.opposite())
                            {
                                if minimum_distance != 1
                                    && (valid_direction == direction
                                        || valid_direction == direction.opposite())
                                {
                                    continue;
                                }

                                for distance in minimum_distance..=maximum_distance {
                                    if distance == maximum_distance
                                        && valid_direction == other_direction
                                    {
                                        continue;
                                    }

                                    let other_speedy_node =
                                        GraphNode::Speedy(new_point, valid_direction, distance);

                                    graph.add_edge(speedy_node, other_speedy_node, heat_loss);
                                }
                            }
                        }

                        if x == 0 && y == 0 {
                            graph.add_edge(GraphNode::Point((x, y)), speedy_node, 0);
                        }

                        graph.add_edge(speedy_node, GraphNode::Point(target_point), heat_loss);
                    }
                }
            }
        }

        let goal_node = (self.heat_loss[0].len() - 1, self.heat_loss.len() - 1);
        let Some(shortest_path) = petgraph::algo::astar::astar(
            &graph,
            GraphNode::Point((0, 0)),
            |node| node == GraphNode::Point(goal_node),
            |(_start, _end, weight)| *weight,
            |_| 0,
        ) else {
            panic!("Could not find shortest path");
        };

        let mut debug_output = vec![vec![0; self.heat_loss[0].len()]; self.heat_loss.len()];

        for (index, item) in shortest_path.1.iter().enumerate() {
            if let GraphNode::Speedy(start_point, direction, distance) = *item {
                for i in 0..=distance {
                    let (x, y) = direction.move_point(start_point, i).unwrap();
                    debug_output[y][x] = index;
                }
            }
        }

        for (i, window) in shortest_path.1.windows(2).enumerate() {
            println!(
                "{i} {window:?} {}",
                graph.edge_weight(window[0], window[1]).unwrap()
            );
        }

        for (y, row) in debug_output.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                let heat_loss = self.heat_loss[y][x];
                if *item != 0 {
                    print!("{}", self.heat_loss[y][x].to_string().red());
                } else {
                    print!("{}", heat_loss);
                }
            }
            println!();
        }

        shortest_path.0 - self.heat_loss[0][0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Right, Self::Down, Self::Left]
            .iter()
            .copied()
    }

    fn all_except(direction: Direction) -> impl Iterator<Item = Self> {
        Self::all().filter(move |&d| d != direction)
    }

    fn move_point(self, (x, y): (usize, usize), amount: usize) -> Option<(usize, usize)> {
        Some(match self {
            Direction::Up => (x, y.checked_sub(amount)?),
            Direction::Right => (x + amount, y),
            Direction::Down => (x, y + amount),
            Direction::Left => (x.checked_sub(amount)?, y),
        })
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GraphNode {
    Speedy((usize, usize), Direction, usize),
    Point((usize, usize)),
}

impl Debug for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Speedy((x, y), direction, distance) => {
                write!(f, "{{({x}, {y}) {direction:?} {distance}}}")
            }
            Self::Point((x, y)) => write!(f, "({x}, {y})"),
        }
    }
}

#[test]
fn given_input() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let city_map = CityMap::parse(input);
    assert_eq!(city_map.minimum_route_cost(3, 10), 94);
    assert_eq!(city_map.minimum_route_cost(1, 3), 102);
}

#[test]
fn part2_given_input() {
    let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

    let city_map = CityMap::parse(input);
    assert_eq!(city_map.minimum_route_cost(3, 10), 71);
}
