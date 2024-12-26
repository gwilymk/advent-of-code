use std::collections::{HashMap, HashSet};

use aoc2024::{get_input, AllPairsExt};
use itertools::Itertools;

fn main() {
    let input = get_input(23);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

fn part2(input: &str) -> String {
    let graph = parse_graph(input);
    let cliques = bron_kerbosch(
        &graph,
        HashSet::new(),
        (0..graph.nodes.len()).collect(),
        HashSet::new(),
    );

    let max_clique = cliques.iter().max_by_key(|clique| clique.len()).unwrap();

    max_clique
        .iter()
        .map(|&n| graph.nodes[n].as_str())
        .sorted()
        .join(",")
}

fn bron_kerbosch(
    graph: &Graph,
    r: HashSet<usize>,
    p: HashSet<usize>,
    x: HashSet<usize>,
) -> Vec<HashSet<usize>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut new_p = p.clone();
    let mut new_x = x.clone();

    let mut result = vec![];

    for &v in &p {
        let neighbours = &graph.connections[v];

        result.extend({
            let mut temp_r = r.clone();
            temp_r.insert(v);

            let temp_p = new_p.intersection(neighbours).copied().collect();
            let temp_x = new_x.intersection(neighbours).copied().collect();

            bron_kerbosch(graph, temp_r, temp_p, temp_x)
        });

        new_p.remove(&v);
        new_x.insert(v);
    }

    result
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

    assert_eq!(part2(input), "co,de,ka,ta");
}
