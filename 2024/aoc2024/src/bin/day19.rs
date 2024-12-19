use std::collections::HashMap;

use aoc2024::get_input;

fn main() {
    let input = get_input(19);
    let (num_matches, total_matches) = get_matches(&input);

    println!("Part 1: {num_matches}");
    println!("Part 2: {total_matches}");
}

struct Nfa {
    state_transitions: HashMap<(usize, char), Vec<usize>>,
}

impl Nfa {
    fn num_matches(&self, pattern: &str) -> usize {
        let mut current_states = HashMap::new();
        current_states.insert(0, 1usize);
        let empty_vec = vec![];

        for c in pattern.chars() {
            let mut new_states: HashMap<usize, usize> = HashMap::new();

            for (&state, &count) in &current_states {
                let transitions = self
                    .state_transitions
                    .get(&(state, c))
                    .unwrap_or(&empty_vec);

                for &transition in transitions {
                    *new_states.entry(transition).or_default() += count;
                }
            }

            current_states = new_states;
        }

        *current_states.get(&0).unwrap_or(&0)
    }
}

fn get_matches(input: &str) -> (usize, usize) {
    let (towel_availablilities, desired_patterns) = input.split_once("\n\n").unwrap();

    // build the nfa
    let mut state_transitions: HashMap<(usize, char), Vec<usize>> = HashMap::new();
    let mut next_available_id = 0;

    for towel in towel_availablilities.split(", ") {
        let mut current_state = 0;
        for (i, stripe) in towel.chars().enumerate() {
            let next_id = if i == towel.len() - 1 {
                0
            } else {
                next_available_id += 1;
                next_available_id
            };

            let transitions = state_transitions
                .entry((current_state, stripe))
                .or_default();
            transitions.push(next_id);

            current_state = next_id;
        }

        assert_eq!(current_state, 0);
    }

    let nfa = Nfa { state_transitions };

    let mut total_possible = 0;
    let mut total_matches = 0;
    for pattern in desired_patterns.split('\n') {
        let num_matches = nfa.num_matches(pattern);

        total_matches += num_matches;
        total_possible += num_matches.min(1);
    }

    (total_possible, total_matches)
}

#[test]
fn given_input() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    assert_eq!(get_matches(input), (6, 16));
}
