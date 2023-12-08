use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", follow_map(input));
    println!("Part 2: {}", follow_all_map(input));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node(String);

struct Map {
    nodes: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut nodes = HashMap::new();

        for line in input.lines() {
            let start = line[0..3].to_owned();
            let left = line[7..10].to_owned();
            let right = line[12..15].to_owned();

            nodes.insert(Node(start), (Node(left), Node(right)));
        }

        Map { nodes }
    }

    fn follow(&self, input: &str) -> usize {
        let mut current = &Node("AAA".to_owned());
        let mut count = 0;

        for c in input.chars().cycle() {
            count += 1;
            let (left, right) = self.nodes.get(current).unwrap();
            if c == 'L' {
                current = left;
            } else {
                current = right;
            }

            if current.0 == "ZZZ" {
                return count;
            }
        }

        unreachable!()
    }

    fn follow_all(&self, input: &str) -> usize {
        let mut current: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|(start, _)| {
                if start.0.ends_with('A') {
                    Some(start)
                } else {
                    None
                }
            })
            .collect();

        let mut count = 0;

        for c in input.chars().cycle() {
            count += 1;

            for node in &mut current {
                let (left, right) = self.nodes.get(node).unwrap();
                if c == 'L' {
                    *node = left;
                } else {
                    *node = right;
                }
            }

            if current.iter().all(|c| c.0.ends_with('Z')) {
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
