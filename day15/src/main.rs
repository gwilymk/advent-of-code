use std::fmt::Debug;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum::<usize>()
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0, |acc, next| (acc + next as usize) * 17 % 256)
}

#[derive(PartialEq, Eq, Hash)]
struct Lense<'a> {
    id: &'a str,
    value: usize,
}

impl<'a> Debug for Lense<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.id, self.value)
    }
}

#[derive(Default)]
struct LenseBox<'a> {
    lenses: Vec<Lense<'a>>,
}

impl<'a> LenseBox<'a> {
    fn focal_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lense)| (i + 1) * lense.value)
            .sum::<usize>()
    }
}

impl<'a> Debug for LenseBox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for lense in &self.lenses {
            write!(f, "{lense:?} ")?;
        }

        Ok(())
    }
}

fn part2(input: &str) -> usize {
    let mut lense_boxes = Vec::with_capacity(256);
    lense_boxes.resize_with(256, LenseBox::default);

    for instruction in input.split(',') {
        let instruction = Instruction::parse(instruction).unwrap();
        match instruction {
            Instruction::Remove(id) => {
                let lense_box = &mut lense_boxes[hash(id)];
                if let Some(box_to_remove) =
                    lense_box.lenses.iter().position(|lense| lense.id == id)
                {
                    lense_box.lenses.remove(box_to_remove);
                }
            }
            Instruction::Add(id, value) => {
                let lense_box = &mut lense_boxes[hash(id)];
                if let Some(current) = lense_box.lenses.iter_mut().find(|lense| lense.id == id) {
                    current.value = value;
                } else {
                    lense_box.lenses.push(Lense { id, value });
                }
            }
        }
    }

    lense_boxes
        .iter()
        .enumerate()
        .map(|(box_number, lense_box)| (box_number + 1) * lense_box.focal_power())
        .sum::<usize>()
}

enum Instruction<'a> {
    Remove(&'a str),
    Add(&'a str, usize),
}

impl<'a> Instruction<'a> {
    fn parse(s: &'a str) -> Result<Self, ()> {
        if let Some(id) = s.strip_suffix('-') {
            Ok(Instruction::Remove(id))
        } else if let Some((id, value)) = s.split_once('=') {
            let Ok(value) = value.parse() else {
                return Err(());
            };

            Ok(Instruction::Add(id, value))
        } else {
            Err(())
        }
    }
}

#[test]
fn part2_test() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    assert_eq!(part2(input), 145);
}
