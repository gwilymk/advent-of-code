use std::{fs, ops::Range};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let almanac = Alamanc::parse(&input);

    println!("Part 1: {}", almanac.closest_seed());
}

struct Alamanc {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Alamanc {
    fn parse(input: &str) -> Self {
        let mut maps = vec![];

        let (seeds, input) = input.split_once('\n').unwrap();
        let (_, seeds) = seeds.split_once(' ').unwrap();
        let seeds = seeds.split(' ').map(|seed| seed.parse().unwrap()).collect();

        let mut current_map = Map::default();
        for line in input.lines() {
            if line.contains(':') {
                maps.push(current_map);
                current_map = Map::default();
            } else if !line.is_empty() {
                current_map.add_line(line);
            }
        }

        maps.push(current_map);

        Alamanc { seeds, maps }
    }

    fn closest_seed(&self) -> usize {
        self.seeds
            .iter()
            .map(|&seed| self.seed_value(seed))
            .min()
            .unwrap()
    }

    fn seed_value(&self, seed: usize) -> usize {
        self.maps
            .iter()
            .fold(seed, |value, map| map.get_value(value))
    }
}

#[derive(Default)]
struct Map {
    offsets: Vec<(Range<usize>, usize)>,
}

impl Map {
    fn add_line(&mut self, input: &str) {
        let bits = input
            .split(' ')
            .map(|bit| bit.parse().unwrap())
            .collect::<Vec<usize>>();

        self.offsets
            .push((bits[1]..(bits[1] + bits[2] + 1), bits[0]));
    }

    fn parse(input: &str) -> Self {
        let mut result = Self::default();

        for line in input.lines() {
            result.add_line(line);
        }

        result
    }

    fn get_value(&self, item: usize) -> usize {
        for (offset_range, dest_start) in &self.offsets {
            if offset_range.contains(&item) {
                return dest_start + item - offset_range.start;
            }
        }

        item
    }
}

#[test]
fn given_input_for_single_map() {
    let input = "50 98 2
52 50 48";

    let map = Map::parse(input);
    assert_eq!(map.get_value(0), 0);
    assert_eq!(map.get_value(48), 48);
    assert_eq!(map.get_value(50), 52);
    assert_eq!(map.get_value(51), 53);
    assert_eq!(map.get_value(96), 98);
    assert_eq!(map.get_value(97), 99);
    assert_eq!(map.get_value(98), 50);
    assert_eq!(map.get_value(99), 51);
}

#[test]
fn given_input_for_entire_almanac() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let almanac = Alamanc::parse(input);
    assert_eq!(almanac.closest_seed(), 35);
}
