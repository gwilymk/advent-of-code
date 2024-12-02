fn main() {
    let input = include_str!("input.txt");

    let parsed = input
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "part 1: {}",
        parsed.iter().filter(|line| is_safe(line)).count()
    );

    println!(
        "part 2: {}",
        parsed.iter().filter(|line| part2(line)).count()
    );
}

fn part2(input: &[i32]) -> bool {
    if is_safe(input) {
        return true;
    }

    for i in 0..input.len() {
        let mut new_input = input.to_vec();
        new_input.remove(i);
        if is_safe(&new_input) {
            return true;
        }
    }

    false
}

fn is_safe(report: &[i32]) -> bool {
    enum Direction {
        Increasing,
        Decreasing,
    }

    let mut current_state = None;

    for window in report.windows(2) {
        let [previous, next] = window else {
            panic!("Not a valid window")
        };

        if !(1..=3).contains(&previous.abs_diff(*next)) {
            return false;
        }

        match current_state {
            None => {
                if previous < next {
                    current_state = Some(Direction::Increasing);
                } else {
                    current_state = Some(Direction::Decreasing);
                }
            }

            Some(Direction::Increasing) => {
                if next < previous {
                    return false;
                }
            }

            Some(Direction::Decreasing) => {
                if next > previous {
                    return false;
                }
            }
        }
    }

    true
}

#[test]
fn given_input() {
    assert!(is_safe(&[7, 6, 4, 2, 1]));
    assert!(!is_safe(&[1, 2, 7, 8, 9]));
    assert!(!is_safe(&[9, 7, 6, 2, 1]));
    assert!(!is_safe(&[1, 3, 2, 4, 5]));
    assert!(!is_safe(&[8, 6, 4, 4, 1]));
    assert!(is_safe(&[1, 3, 6, 7, 9]));

    assert!(part2(&[7, 6, 4, 2, 1]));
    assert!(!part2(&[1, 2, 7, 8, 9]));
    assert!(!part2(&[9, 7, 6, 2, 1]));
    assert!(part2(&[1, 3, 2, 4, 5]));
    assert!(part2(&[8, 6, 4, 4, 1]));
    assert!(part2(&[1, 3, 6, 7, 9]));
}
