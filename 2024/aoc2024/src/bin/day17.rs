use aoc2024::get_input;
use regex::Regex;
use std::fmt::Write;
use std::sync::LazyLock;

fn main() {
    let input = get_input(17);
    let mut machine = Machine::parse(&input);
    println!("Part 1: {}", machine.until_halt());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, enumn::N)]
enum Instruction {
    Adv,
    Blx,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Literal(usize),
    A,
    B,
    C,
}

impl Operand {
    fn from_int(input: usize) -> Self {
        use Operand::*;

        match input {
            0..=3 => Literal(input),
            4 => A,
            5 => B,
            6 => C,
            _ => panic!("Unknown literal {input}"),
        }
    }
}

struct Machine {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    program: Vec<usize>,
}

impl Machine {
    fn parse(input: &str) -> Self {
        static PROGRAM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"Register A: ([0-9]+)\nRegister B: ([0-9]+)\nRegister C: ([0-9]+)\n\nProgram: (.*)").unwrap()
        });

        let (_, [a, b, c, program]) = PROGRAM_REGEX.captures(input).unwrap().extract();

        let program = program.split(',').map(|i| i.parse().unwrap()).collect();

        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
            program,
            pc: 0,
        }
    }

    fn until_halt(&mut self) -> String {
        let mut output = String::new();

        while self.pc < self.program.len() {
            let instr = Instruction::n(self.program[self.pc] as i64).unwrap();
            let op = self.program[self.pc + 1];

            match (instr, op) {
                (Instruction::Adv, op) => self.a >>= self.get_op(op),
                (Instruction::Blx, op) => self.b ^= op,
                (Instruction::Bst, op) => self.b = self.get_op(op) % 8,
                (Instruction::Jnz, op) => {
                    if self.a != 0 {
                        self.pc = op;
                        continue;
                    }
                }
                (Instruction::Bxc, _) => self.b ^= self.c,
                (Instruction::Out, op) => {
                    write!(&mut output, "{},", self.get_op(op) % 8).unwrap();
                }
                (Instruction::Bdv, op) => self.b = self.a >> self.get_op(op),
                (Instruction::Cdv, op) => self.c = self.a >> self.get_op(op),
            }

            self.pc += 2;
        }

        output.pop();
        output
    }

    fn get_op(&self, op: usize) -> usize {
        let op = Operand::from_int(op);

        match op {
            Operand::Literal(v) => v,
            Operand::A => self.a,
            Operand::B => self.b,
            Operand::C => self.c,
        }
    }
}

#[test]
fn given_input() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let mut machine = Machine::parse(input);

    assert_eq!(machine.until_halt(), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn given_input2() {
    let input = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6";

    let mut machine = Machine::parse(input);
    machine.until_halt();
    assert_eq!(machine.b, 1);
}

#[test]
fn given_input3() {
    let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

    let mut machine = Machine::parse(input);
    assert_eq!(machine.until_halt(), "0,1,2");
}

#[test]
fn given_input4() {
    let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let mut machine = Machine::parse(input);
    assert_eq!(machine.until_halt(), "4,2,5,6,7,7,7,7,3,1,0");
    assert_eq!(machine.a, 0);
}

#[test]
fn given_input5() {
    let input = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";

    let mut machine = Machine::parse(input);
    machine.until_halt();
    assert_eq!(machine.b, 26);
}

#[test]
fn given_input6() {
    let input = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";

    let mut machine = Machine::parse(input);
    machine.until_halt();
    assert_eq!(machine.b, 44354);
}
