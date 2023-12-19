use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(input: &str) -> Self {
        let values = input.split(',').map(|value| {
            let (_item, value) = value.split_once('=').unwrap();
            value.trim_end_matches('}').parse::<usize>().unwrap()
        }).collect::<Vec<_>>();

        Self {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }

    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorkflowCheck {
    Greater,
    LessThan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorkflowItemToCheck {
    X,
    M,
    A,
    S,
}

impl WorkflowItemToCheck {
    fn parse(input: char) -> Self {
        match input {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            c => panic!("unknown quality {c}"),
        }
    }

    fn extract_from_part(self, part: &Part) -> usize {
        match self {
            WorkflowItemToCheck::X => part.x,
            WorkflowItemToCheck::M => part.m,
            WorkflowItemToCheck::A => part.a,
            WorkflowItemToCheck::S => part.s,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum WorkflowRule {
    Conditional {
        item: WorkflowItemToCheck,
        check: WorkflowCheck,
        value: usize,
        target: String,
    },
    Unconditional {
        target: String,
    },
}

impl WorkflowRule {
    fn parse(input: &str) -> Vec<Self> {
        input
            .split(',')
            .map(|rule| {
                if !rule.contains(':') {
                    return Self::Unconditional { target: rule.to_owned() };
                }

                let mut iter = rule.chars();
                let item = WorkflowItemToCheck::parse(iter.next().unwrap());
                let check = match iter.next().unwrap() {
                    '<' => WorkflowCheck::LessThan,
                    '>' => WorkflowCheck::Greater,
                    c => panic!("Unknown check {c}"),
                };

                let mut value = 0;
                loop {
                    let c = iter.next().unwrap();
                    if c == ':' {
                        break;
                    }

                    value = value * 10 + c.to_digit(10).unwrap() as usize;
                }

                let target = iter.collect();

                Self::Conditional {
                    item,
                    check,
                    value,
                    target,
                }
            })
            .collect()
    }

    fn target(&self, part: &Part) -> Option<&'_ str> {
        match self {
            WorkflowRule::Conditional { item, check, value, target } => {
                let part_value = item.extract_from_part(part);
                let passes_check = match check {
                    WorkflowCheck::Greater => part_value > *value,
                    WorkflowCheck::LessThan => part_value < *value,
                };

                if passes_check {
                    Some(target)
                } else {
                    None
                }
            },
            WorkflowRule::Unconditional { target } => Some(target),
        }
    }
}

struct Workflow {
    rules: HashMap<String, Vec<WorkflowRule>>,
}

impl Workflow {
    fn parse(input: &str) -> Self {
        let mut rules = HashMap::new();

        for line in input.lines() {
            let (workflow_name, workflow_content) = line.split_once('{').unwrap();
            let rule = WorkflowRule::parse(workflow_content.trim_end_matches('}'));

            rules.insert(workflow_name.to_owned(), rule);
        }

        Self { rules }
    }

    fn result(&self, part: &Part) -> WorkflowResult {
        let mut current_rule = "in";
        while current_rule != "R" && current_rule != "A" {
            let rules = self.rules.get(current_rule).unwrap();
            
            current_rule = rules.iter().filter_map(|rule| rule.target(part)).next().unwrap();
        }

        if current_rule == "A" {
            WorkflowResult::Accept
        } else {
            WorkflowResult::Reject
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkflowResult {
    Accept,
    Reject,
}

fn part1(input: &str) -> usize {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let workflow = Workflow::parse(rules);

    let parts = parts.lines().map(|line| Part::parse(line));
    let accepted_parts = parts.filter(|part| workflow.result(part) == WorkflowResult::Accept);

    accepted_parts.map(|part| part.total_rating()).sum::<usize>()
}

#[test]
fn given_input() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    assert_eq!(part1(input), 19114);
}
