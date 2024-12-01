use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("part 1 {}", part1(&input));
    println!("part 2 {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game = Game::from_str(line).unwrap();

            let is_possible_game = game.steps.iter().all(|step| step.is_possible());

            if is_possible_game {
                game.id
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game = Game::from_str(line).unwrap();
            game.power()
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: usize,
    steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    green: usize,
    blue: usize,
    red: usize,
}

impl Step {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

impl Game {
    fn power(&self) -> usize {
        let red_cubes = self.steps.iter().map(|step| step.red).max().unwrap();
        let blue_cubes = self.steps.iter().map(|step| step.blue).max().unwrap();
        let green_cubes = self.steps.iter().map(|step| step.green).max().unwrap();

        red_cubes * blue_cubes * green_cubes
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static STEPS_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<number>\d+) (?<colour>blue|red|green)").unwrap());

        static GAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?<id>\d+):").unwrap());

        let game_id = GAME_RE
            .captures(s)
            .unwrap()
            .name("id")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let Some((_, steps_part)) = s.split_once(':') else { return Err(()) };

        let steps = steps_part
            .split(';')
            .map(|step| {
                let mut green = 0;
                let mut blue = 0;
                let mut red = 0;

                for colour_part in step.split(',') {
                    let captures = STEPS_RE.captures(colour_part).unwrap();

                    let colour = captures.name("colour").unwrap().as_str();
                    let number = captures.name("number").unwrap().as_str().parse().unwrap();

                    match colour {
                        "blue" => blue = number,
                        "green" => green = number,
                        "red" => red = number,
                        _ => panic!("Unknown colour {colour}"),
                    }
                }

                Step { green, blue, red }
            })
            .collect();

        Ok(Game { id: game_id, steps })
    }
}

#[test]
fn parse_1_game() {
    let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        .parse()
        .unwrap();

    assert_eq!(game.id, 1);
    assert_eq!(
        game.steps,
        &[
            Step {
                blue: 3,
                red: 4,
                green: 0
            },
            Step {
                red: 1,
                green: 2,
                blue: 6
            },
            Step {
                green: 2,
                blue: 0,
                red: 0
            }
        ]
    );
}

#[test]
fn part1_test_input() {
    let value = part1(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    );

    assert_eq!(value, 8);
}

#[test]
fn part2_test_input() {
    let power = part2(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    );

    assert_eq!(power, 2286);
}
