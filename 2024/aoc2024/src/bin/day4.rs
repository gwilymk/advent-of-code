fn main() {
    let input = aoc2024::get_input(4);
    let parsed = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    println!("part1: {}", wordsearch(&parsed));
    println!("part2: {}", x_mas(&parsed));
}

fn x_mas(input: &[Vec<u8>]) -> usize {
    let mut count = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let Some(first_cross) = word_in_direction(input, x, y, 1, 1) else {
                continue;
            };

            let Some(second_cross) = word_in_direction(input, x + 2, y, -1, 1) else {
                continue;
            };

            if (&first_cross == b"MAS" || &first_cross == b"SAM")
                && (&second_cross == b"MAS" || &second_cross == b"SAM")
            {
                count += 1;
            }
        }
    }

    count
}

fn wordsearch(input: &[Vec<u8>]) -> usize {
    let mut count = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == b'X' {
                count += check_xmas(input, x, y);
            }
        }
    }

    count
}

fn check_xmas(input: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let Some(maybe_xmas) = word_in_direction(input, x, y, dx, dy) else {
                continue;
            };

            if &maybe_xmas == b"XMAS" {
                count += 1;
            }
        }
    }

    count
}

fn word_in_direction<const LEN: usize>(
    input: &[Vec<u8>],
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
) -> Option<[u8; LEN]> {
    let mut output = [0; LEN];

    for (i, value) in output.iter_mut().enumerate() {
        *value = *input
            .get(start_y.checked_add_signed(dy * i as isize)?)?
            .get(start_x.checked_add_signed(dx * i as isize)?)?;
    }

    Some(output)
}

#[test]
fn given_input() {
    let parsed = b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        .split(|&c| c == b'\n')
        .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    assert_eq!(wordsearch(&parsed), 18);
    assert_eq!(x_mas(&parsed), 9);
}
