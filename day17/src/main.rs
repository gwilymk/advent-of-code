use std::collections::HashMap;

use petgraph::{algo::dijkstra, Graph};

fn main() {
    println!("Hello, world!");
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

    fn minimum_route_cost(&self) -> usize {
        let mut graph: Graph<usize, usize> = petgraph::Graph::new();

        let mut nodes = vec![];
        nodes.resize_with(self.heat_loss.len(), || vec![None; self.heat_loss[0].len()]);

        for y in 0..self.heat_loss.len() {
            for x in 0..self.heat_loss[0].len() {
                nodes[y][x] = Some(graph.add_node(self.heat_loss[y][x]));
            }
        }

        for y in 0..self.heat_loss.len() {
            for x in 0..self.heat_loss[0].len() {
                for j in -1..=1 {
                    for i in -1..=1 {
                        let target_x = x.checked_add_signed(i);
                        let target_y = y.checked_add_signed(j);

                        let (Some(target_x), Some(target_y)) = (target_x, target_y) else { continue; };
                        if i == 0 && j == 0 {
                            continue;
                        }

                        if target_x >= self.heat_loss[0].len() || target_y >= self.heat_loss.len() {
                            continue;
                        }


                        graph.add_edge(
                            nodes[y][x].unwrap(),
                            nodes[target_y][target_x].unwrap(),
                            self.heat_loss[target_y][target_x],
                        );
                    }
                }
            }
        }

        let goal_node = nodes.last().unwrap().last().unwrap().unwrap();
        let shortest_paths = dijkstra(
            &graph,
            nodes[0][0].unwrap(),
            Some(goal_node),
            |e| *e.weight(),
        );

        *shortest_paths.get(&goal_node).unwrap()
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
    assert_eq!(city_map.minimum_route_cost(), 10);
}
