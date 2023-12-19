use std::{collections::HashMap, rc::Rc};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(input: &str) -> Self {
        let values = input
            .split(',')
            .map(|value| {
                let (_item, value) = value.split_once('=').unwrap();
                value.trim_end_matches('}').parse::<usize>().unwrap()
            })
            .collect::<Vec<_>>();

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

#[derive(Clone)]
struct PartRange {
    x: Rc<[bool; 4000]>,
    m: Rc<[bool; 4000]>,
    a: Rc<[bool; 4000]>,
    s: Rc<[bool; 4000]>,
}

impl std::fmt::Debug for PartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! debug_value {
            ($value: ident) => {
                for &v in self.$value.iter() {
                    if v {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
            };
        }

        write!(f, "x: ")?;
        debug_value!(x);
        write!(f, "\nm: ")?;
        debug_value!(m);
        write!(f, "\na: ")?;
        debug_value!(a);
        write!(f, "\ns: ")?;
        debug_value!(s);
        writeln!(f)?;

        Ok(())
    }
}

impl PartRange {
    fn full() -> Self {
        Self {
            x: Rc::new([true; 4000]),
            m: Rc::new([true; 4000]),
            a: Rc::new([true; 4000]),
            s: Rc::new([true; 4000]),
        }
    }

    fn empty() -> Self {
        Self {
            x: Rc::new([false; 4000]),
            m: Rc::new([false; 4000]),
            a: Rc::new([false; 4000]),
            s: Rc::new([false; 4000]),
        }
    }

    fn total_values(&self) -> usize {
        macro_rules! count_values {
            ($value: ident) => {
                self.$value.iter().filter(|&&value| value).count()
            };
        }

        count_values!(x) * count_values!(m) * count_values!(a) * count_values!(s)
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

    fn extract_from_part_range(self, part: &PartRange) -> &[bool; 4000] {
        match self {
            WorkflowItemToCheck::X => &part.x,
            WorkflowItemToCheck::M => &part.m,
            WorkflowItemToCheck::A => &part.a,
            WorkflowItemToCheck::S => &part.s,
        }
    }

    fn set_part_range(self, part: &mut PartRange, values: Rc<[bool; 4000]>) {
        match self {
            WorkflowItemToCheck::X => part.x = values,
            WorkflowItemToCheck::M => part.m = values,
            WorkflowItemToCheck::A => part.a = values,
            WorkflowItemToCheck::S => part.s = values,
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
                    return Self::Unconditional {
                        target: rule.to_owned(),
                    };
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
            WorkflowRule::Conditional {
                item,
                check,
                value,
                target,
            } => {
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
            }
            WorkflowRule::Unconditional { target } => Some(target),
        }
    }

    // returns ones which pass, ones which don't and where to go
    fn target_range(&self, part_range: &PartRange) -> (PartRange, PartRange, &'_ str) {
        match self {
            WorkflowRule::Conditional {
                item,
                check,
                value,
                target,
            } => {
                let mut accepted_part_range = part_range.clone();
                let mut rejected_part_range = part_range.clone();

                match check {
                    WorkflowCheck::Greater => {
                        let mut accepted =
                            item.extract_from_part_range(&accepted_part_range).clone();
                        let mut rejected =
                            item.extract_from_part_range(&rejected_part_range).clone();
                        for i in 0..*value {
                            accepted[i] = false;
                        }
                        for i in *value..4000 {
                            rejected[i] = false;
                        }
                        item.set_part_range(&mut accepted_part_range, Rc::new(accepted));
                        item.set_part_range(&mut rejected_part_range, Rc::new(rejected));
                    }
                    WorkflowCheck::LessThan => {
                        let mut accepted =
                            item.extract_from_part_range(&accepted_part_range).clone();
                        let mut rejected =
                            item.extract_from_part_range(&rejected_part_range).clone();
                        for i in (*value - 1)..4000 {
                            accepted[i] = false;
                        }
                        for i in 0..(*value - 1) {
                            rejected[i] = false;
                        }
                        item.set_part_range(&mut accepted_part_range, Rc::new(accepted));
                        item.set_part_range(&mut rejected_part_range, Rc::new(rejected));
                    }
                }

                (accepted_part_range, rejected_part_range, target)
            }
            WorkflowRule::Unconditional { target } => {
                (part_range.clone(), PartRange::empty(), target)
            }
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

            current_rule = rules
                .iter()
                .filter_map(|rule| rule.target(part))
                .next()
                .unwrap();
        }

        if current_rule == "A" {
            WorkflowResult::Accept
        } else {
            WorkflowResult::Reject
        }
    }

    fn accepted_parts(&self, start_rule: &str, parts_to_try: PartRange) -> Vec<PartRange> {
        if start_rule == "A" {
            return vec![parts_to_try];
        } else if start_rule == "R" {
            return vec![];
        }

        let mut accepted_parts = vec![];

        let mut current_parts = parts_to_try;
        let current_rule = self.rules.get(start_rule).unwrap();

        for rule in current_rule {
            let (rule_pass, rule_fail, target) = rule.target_range(&current_parts);

            accepted_parts.append(&mut self.accepted_parts(target, rule_pass));
            current_parts = rule_fail;
        }

        accepted_parts
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

    accepted_parts
        .map(|part| part.total_rating())
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let (rules, _parts) = input.split_once("\n\n").unwrap();
    let workflow = Workflow::parse(rules);

    let accepted_parts = workflow.accepted_parts("in", PartRange::full());
    accepted_parts
        .iter()
        .map(|accepted_part| accepted_part.total_values())
        .sum::<usize>()
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
    assert_eq!(part2(input), 167409079868000);
}
