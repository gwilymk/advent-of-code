use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

fn main() {
    let mut machines = SandMachines::parse(include_str!("../input.txt"));

    let part2 = part2(&machines);
    println!("Part 1: {}", part1(&mut machines));
    println!("Part 2: {part2}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Signal {
    HighPulse,
    LowPulse,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HighPulse => write!(f, "+"),
            Self::LowPulse => write!(f, "-"),
        }
    }
}

impl std::ops::Not for Signal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Signal::HighPulse => Signal::LowPulse,
            Signal::LowPulse => Signal::HighPulse,
        }
    }
}

enum ModuleParseType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

struct ModuleParse<'a> {
    name: &'a str,
    module_type: ModuleParseType,
    connections: Vec<&'a str>,
}

impl<'a> ModuleParse<'a> {
    fn parse(line: &'a str) -> Self {
        let (name, connections) = line.split_once(" -> ").unwrap();

        let (module_type, name) = match name.chars().next().unwrap() {
            'b' => (ModuleParseType::Broadcaster, name),
            '%' => (ModuleParseType::FlipFlop, &name[1..]),
            '&' => (ModuleParseType::Conjunction, &name[1..]),
            _ => panic!("Unknown module type {name}"),
        };

        let connections = connections.split(", ").collect();

        Self {
            name,
            module_type,
            connections,
        }
    }
}

#[derive(Clone)]
enum SandMachineType {
    Broadcaster,
    FlipFlop(Signal),
    Conjunction(Vec<Signal>),
}

#[derive(Clone)]
struct SandMachines {
    machines: Vec<SandMachineType>,
    broadcast_index: usize,
    rx_index: Option<usize>,
    graph: petgraph::graphmap::DiGraphMap<usize, ()>,

    conjunction_loops: HashMap<usize, usize>,
}

impl SandMachines {
    fn parse(input: &str) -> Self {
        let parsed_modules = input.lines().map(ModuleParse::parse).collect::<Vec<_>>();
        Self::build(&parsed_modules)
    }

    fn build(module_parse: &[ModuleParse<'_>]) -> Self {
        let name_to_index = module_parse
            .iter()
            .enumerate()
            .map(|(index, module)| (module.name, index))
            .collect::<HashMap<_, _>>();

        let mut graph = petgraph::graphmap::DiGraphMap::new();

        for (index, module_parsed) in module_parse.iter().enumerate() {
            let module_id = index;
            for connection in &module_parsed.connections {
                graph.add_edge(
                    module_id,
                    *name_to_index.get(connection).unwrap_or(&module_parse.len()),
                    (),
                );
            }
        }

        let machines = module_parse
            .iter()
            .enumerate()
            .map(|(index, module)| match module.module_type {
                ModuleParseType::Broadcaster => SandMachineType::Broadcaster,
                ModuleParseType::FlipFlop => SandMachineType::FlipFlop(Signal::LowPulse),
                ModuleParseType::Conjunction => {
                    let number_of_inputs = graph
                        .neighbors_directed(index, petgraph::Direction::Incoming)
                        .count();
                    SandMachineType::Conjunction(vec![Signal::LowPulse; number_of_inputs])
                }
            })
            .collect::<Vec<_>>();

        let broadcast_index = machines
            .iter()
            .position(|machine| matches!(machine, SandMachineType::Broadcaster))
            .unwrap();

        let rx_index = Some(module_parse.len());

        Self {
            graph,
            machines,
            broadcast_index,
            rx_index,

            conjunction_loops: HashMap::new(),
        }
    }

    fn push_button(&mut self, loop_count: usize) -> (usize, usize) {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back((self.broadcast_index, Signal::LowPulse, self.broadcast_index));

        let mut low_pulses_sent = 0;
        let mut high_pulses_sent = 0;

        while let Some((module_index, pulse, sender)) = pulse_queue.pop_front() {
            match pulse {
                Signal::HighPulse => high_pulses_sent += 1,
                Signal::LowPulse => low_pulses_sent += 1,
            }

            match &mut self.machines.get_mut(module_index) {
                Some(SandMachineType::Broadcaster) => {
                    for out in self
                        .graph
                        .neighbors_directed(module_index, petgraph::Direction::Outgoing)
                    {
                        pulse_queue.push_back((out, pulse, module_index));
                    }
                }
                Some(SandMachineType::FlipFlop(ref mut state)) => {
                    if pulse == Signal::LowPulse {
                        *state = !*state;
                        for out in self
                            .graph
                            .neighbors_directed(module_index, petgraph::Direction::Outgoing)
                        {
                            pulse_queue.push_back((out, *state, module_index));
                        }
                    }
                }
                Some(SandMachineType::Conjunction(ref mut state)) => {
                    // find out who sent this
                    let sender = self
                        .graph
                        .neighbors_directed(module_index, petgraph::Direction::Incoming)
                        .position(|neighbour| neighbour == sender)
                        .unwrap();

                    state[sender] = pulse;

                    let pulse_to_send = if state.iter().all(|&signal| signal == Signal::HighPulse) {
                        self.conjunction_loops
                            .entry(module_index)
                            .or_insert(loop_count);
                        Signal::LowPulse
                    } else {
                        Signal::HighPulse
                    };

                    for out in self
                        .graph
                        .neighbors_directed(module_index, petgraph::Direction::Outgoing)
                    {
                        pulse_queue.push_back((out, pulse_to_send, module_index));
                    }
                }
                None => {}
            }
        }

        (low_pulses_sent, high_pulses_sent)
    }

    fn presses_until_rx(&self) -> usize {
        let mut test_instance = self.clone();

        let input_to_rx = self
            .graph
            .neighbors_directed(self.rx_index.unwrap(), petgraph::Direction::Incoming)
            .next()
            .unwrap();

        let input_to_input = self
            .graph
            .neighbors_directed(input_to_rx, petgraph::Direction::Incoming)
            .flat_map(|neighbour| {
                self.graph
                    .neighbors_directed(neighbour, petgraph::Direction::Incoming)
            })
            .collect::<Vec<_>>();

        let mut input_times = HashMap::with_capacity(input_to_input.len());

        for button_push in 1.. {
            test_instance.push_button(button_push);

            for &input in &input_to_input {
                if let Some(loop_) = test_instance.conjunction_loops.get(&input) {
                    input_times.entry(input).or_insert(*loop_);
                }
            }

            if input_times.len() == input_to_input.len() {
                break;
            }
        }

        input_times.values().copied().fold(1, lcm)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn part1(machines: &mut SandMachines) -> usize {
    let mut total_high = 0;
    let mut total_low = 0;

    for i in 1..1000 {
        let (lows, highs) = machines.push_button(i);
        total_high += highs;
        total_low += lows;
    }

    total_high * total_low
}

fn part2(machines: &SandMachines) -> usize {
    machines.presses_until_rx()
}

#[test]
fn given_input1() {
    let mut machine = SandMachines::parse(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
    );

    assert_eq!(machine.push_button(1), (8, 4));
}

#[test]
fn given_input2() {
    let mut machine = SandMachines::parse(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    );

    assert_eq!(part1(&mut machine), 11687500);
}
