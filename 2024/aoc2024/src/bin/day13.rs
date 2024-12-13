use aoc2024::{get_input, Vector2D};

fn main() {
    let input = get_input(13);
    println!("Part 1: {}", part1(&input));
}

#[derive(Clone, Debug)]
struct ClawMachineConfig {
    a: Vector2D<u32>,
    b: Vector2D<u32>,
    prize: Vector2D<u32>,
}

fn minimum_cost_up_to_100(config: &ClawMachineConfig) -> Option<usize> {
    (0..=100)
        .flat_map(|a| (0..=100).map(move |b| (a, b)))
        .filter_map(|(a, b)| {
            if config.a * a + config.b * b == config.prize {
                Some((3 * a + b) as usize)
            } else {
                None
            }
        })
        .min()
}

fn part1(input: &str) -> usize {
    let button_line = regex::Regex::new(r"X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_line = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    input
        .split("\n\n")
        .map(|chunk| {
            let lines = chunk.split('\n').collect::<Vec<_>>();

            let (_, [ax, ay]) = button_line.captures(lines[0]).unwrap().extract();
            let (_, [bx, by]) = button_line.captures(lines[1]).unwrap().extract();
            let (_, [px, py]) = prize_line.captures(lines[2]).unwrap().extract();

            let config = ClawMachineConfig {
                a: Vector2D::new(ax.parse().unwrap(), ay.parse().unwrap()),
                b: Vector2D::new(bx.parse().unwrap(), by.parse().unwrap()),
                prize: Vector2D::new(px.parse().unwrap(), py.parse().unwrap()),
            };

            minimum_cost_up_to_100(&config).unwrap_or(0)
        })
        .sum()
}

#[test]
fn given_input() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    assert_eq!(part1(&input), 480);
}
