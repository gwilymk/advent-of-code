use aoc2024::get_input;

fn main() {
    let input = get_input(13);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Debug)]
struct ClawMachineConfig {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn minimum_cost_up_to_100(config: &ClawMachineConfig) -> Option<usize> {
    (0..=100)
        .flat_map(|a| (0..=100).map(move |b| (a, b)))
        .filter_map(|(a, b)| {
            if (
                config.a.0 * a + config.b.0 * b,
                config.a.1 * a + config.b.1 * b,
            ) == config.prize
            {
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
                a: (ax.parse().unwrap(), ay.parse().unwrap()),
                b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (px.parse().unwrap(), py.parse().unwrap()),
            };

            minimum_cost_up_to_100(&config).unwrap_or(0)
        })
        .sum()
}

fn part2(input: &str) -> usize {
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
                a: (ax.parse().unwrap(), ay.parse().unwrap()),
                b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (
                    px.parse::<u64>().unwrap() + 10000000000000,
                    py.parse::<u64>().unwrap() + 10000000000000,
                ),
            };

            minimum_cost(&config).unwrap_or(0)
        })
        .sum()
}

fn minimum_cost(config: &ClawMachineConfig) -> Option<usize> {
    let a0 = config.a.0 as i64;
    let b0 = config.b.0 as i64;
    let a1 = config.a.1 as i64;
    let b1 = config.b.1 as i64;
    let p0 = config.prize.0 as i64;
    let p1 = config.prize.1 as i64;

    let det = a0 * b1 - b0 * a1;
    if det == 0 {
        return None;
    }

    let a_mul_det = b1 * p0 - b0 * p1;
    let b_mul_det = -a1 * p0 + a0 * p1;

    if a_mul_det % det != 0 || b_mul_det % det != 0 {
        return None;
    }

    let a = a_mul_det / det;
    let b = b_mul_det / det;

    if a < 0 || b < 0 {
        return None;
    }

    Some((3 * a + b) as usize)
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

    assert_eq!(part1(input), 480);

    assert_eq!(part2(input), 875318608908);
}
