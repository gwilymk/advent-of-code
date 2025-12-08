use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use aoc2025::AllPairsExt;

fn main() {
    let input = aoc2025::get_input(8);

    let (positions, sorted_pairs) = parse(&input);
    println!("Part 1: {}", part1(&sorted_pairs, 1000));
    println!("Part 2: {}", part2(&positions, &sorted_pairs));
}

fn part1(sorted_pairs: &[(usize, usize)], connections: usize) -> usize {
    // map id to the circuit it's in
    let mut node_to_circuit = HashMap::new();
    let mut next_circuit_id = 0usize;

    let mut circuits: HashMap<usize, HashSet<_>> = HashMap::new();

    for &(a, b) in sorted_pairs.iter().take(connections) {
        let circuit_a = node_to_circuit.get(&a).copied();
        let circuit_b = node_to_circuit.get(&b).copied();

        match (circuit_a, circuit_b) {
            (None, None) => {
                node_to_circuit.insert(a, next_circuit_id);
                node_to_circuit.insert(b, next_circuit_id);
                let c = circuits.entry(next_circuit_id).or_default();
                c.insert(a);
                c.insert(b);

                next_circuit_id += 1;
            }
            (Some(circuit_a), None) => {
                node_to_circuit.insert(b, circuit_a);
                circuits.entry(circuit_a).or_default().insert(b);
            }
            (None, Some(circuit_b)) => {
                node_to_circuit.insert(a, circuit_b);
                circuits.entry(circuit_b).or_default().insert(a);
            }
            (Some(circuit_a), Some(circuit_b)) => {
                // need to combine these...
                if circuit_a == circuit_b {
                    continue;
                }

                let [Some(island_a), Some(island_b)] =
                    circuits.get_disjoint_mut([&circuit_a, &circuit_b])
                else {
                    panic!("Couldn't find entries");
                };

                island_b.drain().for_each(|b_entry| {
                    island_a.insert(b_entry);
                    node_to_circuit.insert(b_entry, circuit_a);
                });

                circuits.remove(&circuit_b);
            }
        }
    }

    let mut island_sizes = circuits
        .values()
        .map(|c| Reverse(c.len()))
        .collect::<Vec<_>>();
    island_sizes.sort_unstable();

    island_sizes.iter().take(3).map(|s| s.0).product::<usize>()
}

fn part2(positions: &[[u64; 3]], sorted_pairs: &[(usize, usize)]) -> u64 {
    // map id to the circuit it's in
    let mut node_to_circuit = HashMap::new();
    let mut next_circuit_id = 0usize;

    let mut circuits: HashMap<usize, HashSet<_>> = HashMap::new();

    for &(a, b) in sorted_pairs {
        let circuit_a = node_to_circuit.get(&a).copied();
        let circuit_b = node_to_circuit.get(&b).copied();

        match (circuit_a, circuit_b) {
            (None, None) => {
                node_to_circuit.insert(a, next_circuit_id);
                node_to_circuit.insert(b, next_circuit_id);
                let c = circuits.entry(next_circuit_id).or_default();
                c.insert(a);
                c.insert(b);

                next_circuit_id += 1;
            }
            (Some(circuit_a), None) => {
                node_to_circuit.insert(b, circuit_a);
                circuits.entry(circuit_a).or_default().insert(b);
            }
            (None, Some(circuit_b)) => {
                node_to_circuit.insert(a, circuit_b);
                circuits.entry(circuit_b).or_default().insert(a);
            }
            (Some(circuit_a), Some(circuit_b)) => {
                // need to combine these...
                if circuit_a == circuit_b {
                    continue;
                }

                let [Some(island_a), Some(island_b)] =
                    circuits.get_disjoint_mut([&circuit_a, &circuit_b])
                else {
                    panic!("Couldn't find entries");
                };

                island_b.drain().for_each(|b_entry| {
                    island_a.insert(b_entry);
                    node_to_circuit.insert(b_entry, circuit_a);
                });

                circuits.remove(&circuit_b);
            }
        }

        if circuits.len() == 1 && circuits.values().next().unwrap().len() == positions.len() {
            // this was the last one
            return positions[a][0] * positions[b][0];
        }
    }

    unreachable!("Never formed 1 circuit.... somehow");
}

fn parse(input: &str) -> (Vec<[u64; 3]>, Vec<(usize, usize)>) {
    let positions: Vec<[u64; 3]> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut all_pairs = (0..positions.len())
        .collect::<Vec<_>>()
        .all_pairs()
        .collect::<Vec<_>>();

    all_pairs.sort_unstable_by_key(|pair| {
        positions[pair.0]
            .iter()
            .zip(&positions[pair.1])
            .map(|(a, b)| (a.max(b) - a.min(b)).pow(2))
            .sum::<u64>()
    });

    (positions, all_pairs)
}

#[cfg(test)]
const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[test]
fn test_part1() {
    let (_, sorted_pairs) = parse(TEST_INPUT);

    assert_eq!(part1(&sorted_pairs, 10), 40);
}

#[test]
fn test_part2() {
    let (positions, sorted_pairs) = parse(TEST_INPUT);

    assert_eq!(part2(&positions, &sorted_pairs), 25272);
}
