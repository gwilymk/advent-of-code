use std::{fs, net, ops::Range};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let almanac1 = Almanac::parse(&input, SeedParsingStrategy::Values);

    println!("Part 1: {}", almanac1.closest_seed());

    let almanac2 = Almanac::parse(&input, SeedParsingStrategy::Range);
    println!("Part 2: {}", almanac2.closest_seed());
}

struct Almanac {
    seeds: Vec<Range<usize>>,
    maps: Vec<Map>,
}

enum SeedParsingStrategy {
    Values,
    Range,
}

impl Almanac {
    fn parse(input: &str, seed_parsing_strategy: SeedParsingStrategy) -> Self {
        let mut maps = vec![];

        let (seeds, input) = input.split_once('\n').unwrap();
        let (_, seeds) = seeds.split_once(' ').unwrap();
        let seeds = match seed_parsing_strategy {
            SeedParsingStrategy::Values => seeds
                .split(' ')
                .map(|seed| {
                    let value = seed.parse().unwrap();
                    value..(value + 1)
                })
                .collect(),

            SeedParsingStrategy::Range => seeds
                .split(' ')
                .collect::<Vec<_>>()
                .chunks_exact(2)
                .map(|seed_range| {
                    let start = seed_range[0].parse().unwrap();
                    let count: usize = seed_range[1].parse().unwrap();
                    start..(start + count + 1)
                })
                .collect(),
        };

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

        Almanac { seeds, maps }
    }

    fn closest_seed(&self) -> usize {
        self.seeds
            .iter()
            .flat_map(|seed_range| self.seed_range(seed_range))
            .map(|range| range.start)
            .min()
            .unwrap()
    }

    fn seed_value(&self, seed: usize) -> usize {
        self.maps
            .iter()
            .fold(seed, |value, map| map.get_value(value))
    }

    fn seed_range(&self, seed_range: &Range<usize>) -> Vec<Range<usize>> {
        self.maps
            .iter()
            .fold(vec![seed_range.clone()], |seed_ranges, map| {
                seed_ranges
                    .iter()
                    .flat_map(|seed_range| map.get_ranges(seed_range))
                    .collect()
            })
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

        self.offsets.push((bits[1]..(bits[1] + bits[2]), bits[0]));
    }

    #[cfg(test)]
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

    fn get_ranges(&self, range: &Range<usize>) -> Vec<Range<usize>> {
        let mut offsets = self.offsets.clone();
        offsets.sort_by_key(|o| o.0.start);

        let mut result = vec![];
        let mut i = range.start;

        while i < range.end {
            if let Some((r, dest_start)) = offsets.iter().find(|o| o.0.contains(&i)) {
                let intersection = r.start.max(range.start)..r.end.min(range.end);
                let start = dest_start + i - r.start;

                result.push(start..(start + intersection.end - intersection.start));
                i = intersection.end;
            } else if let Some(n) = offsets.iter().rposition(|(o, _)| o.contains(&i)) {
                // this is the last one which contains i, so we actually want the next one.
                if let Some((next, _)) = offsets.get(n + 1) {
                    result.push(i..next.start);
                    i = next.start;
                } else {
                    result.push(i..range.end);
                    i = range.end;
                }
            } else {
                result.push(i..range.end);
                i = range.end;
            }
        }

        assert_eq!(
            result.iter().map(|r| r.end - r.start).sum::<usize>(),
            range.end - range.start
        );

        result
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

    let almanac = Almanac::parse(input, SeedParsingStrategy::Values);
    assert_eq!(almanac.closest_seed(), 35);
}

#[test]
fn given_input_for_entire_almanac_seed_ranges() {
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

    let almanac = Almanac::parse(input, SeedParsingStrategy::Range);
    assert_eq!(almanac.closest_seed(), 46);
}
