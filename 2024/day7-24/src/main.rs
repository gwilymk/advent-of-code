fn main() {
    println!("Part 1: {}", num_solvable(include_str!("input.txt")));
}

fn solvable_equation(expected_result: i64, inputs: &[i64]) -> bool {
    for i in 0..(1 << (inputs.len() - 1)) {
        let mut current = inputs[0];

        for bit in 0..(inputs.len() - 1) {
            if i & (1 << bit) == 0 {
                current += inputs[bit + 1];
            } else {
                current *= inputs[bit + 1];
            }
        }

        if current == expected_result {
            return true;
        }
    }

    false
}

fn num_solvable(input: &str) -> i64 {
    input
        .split('\n')
        .filter_map(|line| {
            let (answer, question) = line.split_once(": ").unwrap();
            let answer = answer.parse::<_>().unwrap();
            let question = question
                .split(' ')
                .map(|n| n.parse::<_>().unwrap())
                .collect::<Vec<_>>();

            if solvable_equation(answer, &question) {
                Some(answer)
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn given_input() {
    assert_eq!(
        num_solvable(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        ),
        3749
    );
}
