use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("day 1 {}", calibration(&content));
    println!("day 2 {}", calibration2(&content));
}

fn calibration(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_number: u32 = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last_number: u32 = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();

            first_number * 10 + last_number
        })
        .sum::<u32>()
}

const NUMBER_WORDS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calibration2(input: &str) -> u32 {
    input.lines().map(value2_for_line).sum::<u32>()
}

fn value2_for_line(line: &str) -> u32 {
    let chars = line.chars().collect::<Vec<_>>();

    let first_digit_index = chars.iter().position(|c| c.is_ascii_digit());
    let first_word_index = find_word_number(line);

    let last_digit_index = chars.iter().rposition(|c| c.is_ascii_digit());
    let last_word_index = find_word_number_backwards(line);

    let first_digit = match (first_digit_index, first_word_index) {
        (Some(i), None) => chars[i].to_digit(10).unwrap(),
        (None, Some((_, number))) => number,
        (Some(i1), Some((i2, number))) => {
            if i1 < i2 {
                chars[i1].to_digit(10).unwrap()
            } else {
                number
            }
        }
        (None, None) => unimplemented!(),
    };

    let last_digit = match (last_digit_index, last_word_index) {
        (Some(i), None) => chars[i].to_digit(10).unwrap(),
        (None, Some((_, number))) => number,
        (Some(i1), Some((i2, number))) => {
            if i1 > i2 {
                chars[i1].to_digit(10).unwrap()
            } else {
                number
            }
        }
        (None, None) => unimplemented!(),
    };

    first_digit * 10 + last_digit
}

fn find_word_number(line: &str) -> Option<(usize, u32)> {
    for i in 0..line.len() {
        for (number, word) in NUMBER_WORDS.iter().enumerate() {
            if line[i..].starts_with(word) {
                return Some((i, number as u32));
            }
        }
    }

    None
}

fn find_word_number_backwards(line: &str) -> Option<(usize, u32)> {
    for i in (0..line.len()).rev() {
        for (number, word) in NUMBER_WORDS.iter().enumerate() {
            if line[i..].starts_with(word) {
                return Some((i, number as u32));
            }
        }
    }

    None
}

#[test]
fn find_word_number_test() {
    assert_eq!(find_word_number("eighttwothree"), Some((0, 8)));
    assert_eq!(find_word_number_backwards("eighttwothree"), Some((8, 3)));
}

#[test]
fn given_input1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(calibration(input), 142);
}

#[test]
fn given_input2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(calibration2(input), 281);
}

#[test]
fn part_of_input2() {
    assert_eq!(calibration2("eighttwothree"), 83);
}
