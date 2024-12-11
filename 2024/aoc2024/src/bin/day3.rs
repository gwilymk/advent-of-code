use regex::Regex;

fn main() {
    let input = aoc2024::get_input(3);
    println!("part1: {}", execute(&input, false));
    println!("part2: {}", execute(&input, true));
}

fn execute(input: &str, conditional_parsing: bool) -> i64 {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)|do\\(\\)|don't\\(\\)").unwrap();

    let mut enabled = true;
    let mut result = 0;

    for capture in re.captures_iter(input) {
        let line = capture.get(0).unwrap().as_str();

        if line == "do()" {
            enabled = true;
        } else if line == "don't()" {
            enabled = false;
        } else if !conditional_parsing || enabled {
            let a = capture.get(1).unwrap().as_str();
            let b = capture.get(2).unwrap().as_str();

            result += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
        }
    }

    result
}

#[test]
fn given_input() {
    assert_eq!(
        execute(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            false
        ),
        161
    );

    assert_eq!(
        execute(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            true
        ),
        48
    );
}
