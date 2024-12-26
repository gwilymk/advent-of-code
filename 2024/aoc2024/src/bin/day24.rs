use std::{cmp::Reverse, collections::HashMap};

use aoc2024::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(24);
    println!("Part 1: {}", part1(&input));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Gate::And => a && b,
            Gate::Or => a || b,
            Gate::Xor => a ^ b,
        }
    }
}

fn part1(input: &str) -> u64 {
    struct Connection<'a> {
        input1: &'a str,
        input2: &'a str,
        gate: Gate,
        output: &'a str,
    }

    let mut values: HashMap<&str, bool> = HashMap::new();

    let (starts, gates) = input.split_once("\n\n").unwrap();
    for start in starts.split('\n') {
        let (name, value) = start.split_once(": ").unwrap();

        values.insert(name, value == "1");
    }

    let gate_regex = regex::Regex::new("(\\w+) (XOR|OR|AND) (\\w+) -> (\\w+)").unwrap();

    let gates = gates
        .split('\n')
        .map(|gate| {
            let (_, [first, gate, second, output]) = gate_regex.captures(gate).unwrap().extract();

            let gate = match gate {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => panic!("Unknown gate {gate}"),
            };

            Connection {
                input1: first,
                input2: second,
                gate,
                output,
            }
        })
        .collect::<Vec<_>>();

    loop {
        let mut did_something = false;

        for gate in &gates {
            let input1 = values.get(gate.input1).copied();
            let input2 = values.get(gate.input2).copied();
            let output = values.get(gate.output).copied();

            if let (Some(input1), Some(input2), None) = (input1, input2, output) {
                values.insert(gate.output, gate.gate.apply(input1, input2));
                did_something = true;
            }
        }

        if !did_something {
            break;
        }
    }

    // collect all the z's
    values
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_by_key(|kv| Reverse(kv.0))
        .fold(0u64, |curr, (_, next)| {
            (curr << 1) | if next { 1 } else { 0 }
        })
}

#[test]
fn given_input() {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    assert_eq!(part1(input), 2024);
}
