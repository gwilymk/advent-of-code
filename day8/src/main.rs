use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", follow_map(input));
    println!("Part 2: {}", follow_all_map(input));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node(String);

struct Map {
    nodes: Vec<Node>,

    map: Vec<(usize, usize)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut nodes = HashMap::new();
        let mut node_to_id = HashMap::new();

        let mut nodes_vec = vec![];

        for line in input.lines() {
            let start = line[0..3].to_owned();
            let left = line[7..10].to_owned();
            let right = line[12..15].to_owned();

            let start = Node(start);
            nodes.insert(start.clone(), (Node(left), Node(right)));
            node_to_id.insert(start.clone(), nodes_vec.len());
            nodes_vec.push(start);
        }

        let mut map = vec![Default::default(); nodes.len()];

        for (start, (left, right)) in nodes.iter() {
            map[node_to_id[start]] = (node_to_id[left], node_to_id[right]);
        }

        Map {
            nodes: nodes_vec,
            map,
        }
    }

    fn follow(&self, input: &str) -> usize {
        let mut current = self.nodes.iter().position(|x| x.0 == "AAA").unwrap();
        let mut count = 0;

        for c in input.chars().cycle() {
            count += 1;
            let (left, right) = self.map[current];
            if c == 'L' {
                current = left;
            } else {
                current = right;
            }

            if self.nodes[current].0 == "ZZZ" {
                return count;
            }
        }

        unreachable!()
    }

    fn follow_all(&self, input: &str) -> usize {
        let starting_positions: Vec<_> = self
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(id, start)| {
                if start.0.ends_with('A') {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();

        let mut cycle_lengths = vec![];

        // want to find the length of each loop for each starting position
        for starting_position in starting_positions {
            let mut current_position = starting_position;

            let mut seen_positions = vec![vec![0; input.len()]; self.nodes.len()];

            for (time, (i, &c)) in input.as_bytes().iter().enumerate().cycle().enumerate() {
                if seen_positions[current_position][i] != 0 {
                    // we've been here before... so this is the loop length
                    let cycle_length = time + 1 - seen_positions[current_position][i];
                    cycle_lengths.push(cycle_length);
                    break;
                }

                seen_positions[current_position][i] = time + 1;

                let (left, right) = self.map[current_position];
                if c == b'L' {
                    current_position = left;
                } else {
                    current_position = right;
                }
            }
        }

        cycle_lengths
            .iter()
            .fold(1, |curr, cycle| num::integer::lcm(curr, *cycle))
    }
}

fn follow_map(input: &str) -> usize {
    let (directions, map) = input.split_once("\n\n").unwrap();

    let map = Map::parse(map);

    map.follow(directions)
}

fn follow_all_map(input: &str) -> usize {
    let (directions, map) = input.split_once("\n\n").unwrap();

    let map = Map::parse(map);

    map.follow_all(directions)
}

#[test]
fn given_part1_input() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(follow_map(input), 6);
}

#[test]
fn given_part2_input() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(follow_all_map(input), 6);
}
