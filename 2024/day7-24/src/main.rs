fn main() {
    println!("Part 1: {}", num_solvable(include_str!("input.txt"), false));
    println!("Part 2: {}", num_solvable(include_str!("input.txt"), true));
}

fn recursive_solvable_equation(expected_result: i64, inputs: &[i64], include_concat: bool) -> bool {
    if inputs.len() == 1 {
        return expected_result == inputs[0];
    }

    let test_value = inputs[inputs.len() - 1];
    if expected_result % test_value == 0
        && recursive_solvable_equation(
            expected_result / test_value,
            &inputs[..inputs.len() - 1],
            include_concat,
        )
    {
        return true;
    }

    if expected_result >= test_value
        && recursive_solvable_equation(
            expected_result - test_value,
            &inputs[..inputs.len() - 1],
            include_concat,
        )
    {
        return true;
    }

    if include_concat {
        let expected_result_as_str = expected_result.to_string();
        let concat_amount = test_value.to_string();

        if expected_result_as_str.ends_with(&concat_amount) {
            let without_concat =
                &expected_result_as_str[..expected_result_as_str.len() - concat_amount.len()];

            if !without_concat.is_empty()
                && recursive_solvable_equation(
                    without_concat.parse().unwrap(),
                    &inputs[..inputs.len() - 1],
                    include_concat,
                )
            {
                return true;
            }
        }
    }

    false
}

fn num_solvable(input: &str, include_concat: bool) -> i64 {
    input
        .split('\n')
        .filter_map(|line| {
            let (answer, question) = line.split_once(": ").unwrap();
            let answer = answer.parse::<_>().unwrap();
            let question = question
                .split(' ')
                .map(|n| n.parse::<_>().unwrap())
                .collect::<Vec<_>>();

            if recursive_solvable_equation(answer, &question, include_concat) {
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
292: 11 6 16 20",
            false
        ),
        3749
    );

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
292: 11 6 16 20",
            true
        ),
        11387
    );
}
