use rand::seq::SliceRandom;
use std::collections::HashSet;

use petgraph::{
    algo::{kosaraju_scc, min_spanning_tree},
    data::FromElements,
    prelude::*,
};

fn main() {
    let input = include_str!("../input.txt");
    let snow_machine = SnowMachine::parse(input);

    println!("Part 1: {}", snow_machine.split_in_two());
}

struct SnowMachine<'a> {
    connections: UnGraphMap<&'a str, ()>,
}

impl<'a> SnowMachine<'a> {
    fn parse(input: &'a str) -> Self {
        let mut graph = UnGraphMap::new();

        for line in input.lines() {
            let (start, connections) = line.split_once(": ").unwrap();

            for connection in connections.split(' ') {
                graph.add_edge(start, connection, ());
            }
        }

        Self { connections: graph }
    }

    fn split_in_two(&self) -> usize {
        let mut rng = rand::thread_rng();

        loop {
            let mut temp_graph = UnGraphMap::new();
            let edge_count = self.connections.edge_count();
            let mut random_weights = (1..=edge_count).collect::<Vec<_>>();
            random_weights.shuffle(&mut rng);

            for ((from, to, _), weight) in self.connections.all_edges().zip(random_weights.iter()) {
                temp_graph.add_edge(from, to, weight);
            }

            let tree = min_spanning_tree(&temp_graph);
            let mut graph_result: Graph<&str, &usize, Undirected, u32> = Graph::from_elements(tree);

            // find the heaviest edge and remove that
            let heaviest = graph_result
                .edge_references()
                .max_by_key(|edge| edge.weight())
                .unwrap();

            graph_result.remove_edge(heaviest.id());

            // get the two connected components
            let connected_components = kosaraju_scc(&graph_result);

            assert_eq!(connected_components.len(), 2);

            let first_batch = &connected_components[0];
            let second_batch = connected_components[1]
                .iter()
                .map(|&node| *graph_result.node_weight(node).unwrap())
                .collect::<HashSet<_>>();

            let mut edges_removed = vec![];

            for &item in first_batch {
                let node_name = *graph_result.node_weight(item).unwrap();
                let neighbours_in_second_batch = self
                    .connections
                    .neighbors(node_name)
                    .filter(|neighbour| second_batch.contains(neighbour));

                for neighbour_in_second_batch in neighbours_in_second_batch {
                    edges_removed.push((node_name, neighbour_in_second_batch));
                }
            }

            if edges_removed.len() == 3 {
                return first_batch.len() * second_batch.len();
            }
        }
    }
}

#[test]
fn given_input() {
    let snow_machine = SnowMachine::parse(
        "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
    );

    assert_eq!(snow_machine.split_in_two(), 54);
}
