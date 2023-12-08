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
    end: Vec<bool>,

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
        let mut end = vec![false; nodes.len()];

        for (start, (left, right)) in nodes.iter() {
            map[node_to_id[start]] = (node_to_id[left], node_to_id[right]);
            if start.0.ends_with('Z') {
                end[node_to_id[start]] = true;
            }
        }

        Map {
            nodes: nodes_vec,
            map,
            end,
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
        let mut current: Vec<_> = self
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

        let mut count = 0;

        for c in input.chars().cycle() {
            count += 1;

            if count % 10000000 == 0 {
                println!("{count}");
            }

            for node in &mut current {
                let (left, right) = self.map[*node];
                if c == 'L' {
                    *node = left;
                } else {
                    *node = right;
                }
            }

            if current.iter().all(|&c| self.end[c]) {
                return count;
            }
        }

        todo!()
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
