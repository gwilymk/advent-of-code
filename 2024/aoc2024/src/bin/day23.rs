use std::collections::{HashMap, HashSet};

use aoc2024::{get_input, AllPairsExt};

fn main() {
    let input = get_input(23);
    println!("Part 1: {}", part1(&input));
}

struct Graph {
    nodes: Vec<String>,
    connections: Vec<HashSet<usize>>,
}

fn parse_graph(input: &str) -> Graph {
    let mut lookup: HashMap<&str, usize> = HashMap::new();
    let mut nodes = vec![];

    let mut connections: Vec<HashSet<usize>> = vec![];

    for line in input.split('\n') {
        let (left, right) = line.split_once('-').unwrap();

        let left_index = if let Some(idx) = lookup.get(left) {
            *idx
        } else {
            let idx = nodes.len();
            lookup.insert(left, idx);
            nodes.push(left.to_string());
            idx
        };

        let right_index = if let Some(idx) = lookup.get(right) {
            *idx
        } else {
            let idx = nodes.len();
            lookup.insert(right, idx);
            nodes.push(right.to_string());
            idx
        };

        connections.resize(
            (left_index + 1).max(right_index + 1).max(connections.len()),
            Default::default(),
        );

        connections[left_index].insert(right_index);
        connections[right_index].insert(left_index);
    }

    Graph { nodes, connections }
}

fn groups_of_size_3(graph: &Graph) -> Vec<[usize; 3]> {
    let mut results = vec![];

    for i in 0..graph.nodes.len() {
        let node_connections: Vec<_> = graph.connections[i].iter().copied().collect();

        for (first, second) in node_connections.all_pairs() {
            if graph.connections[first].contains(&second) {
                results.push([i, first, second]);
            }
        }
    }

    results
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);
    let all_groups_of_size_3 = groups_of_size_3(&graph);

    let mut count = 0;
    for group in all_groups_of_size_3 {
        if group.iter().any(|&idx| graph.nodes[idx].starts_with('t')) {
            count += 1;
        }
    }

    count / 3
}

#[test]
fn given_input() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    assert_eq!(part1(input), 7);
}
