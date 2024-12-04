fn main() {
    let parsed = include_bytes!("input.txt")
        .split(|&c| c == b'\n')
        .map(|line| line.to_vec())
        .collect::<Vec<_>>();

    println!("part1: {}", wordsearch(&parsed));
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

fn word_in_direction(
    input: &[Vec<u8>],
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
) -> Option<[u8; 4]> {
    let mut output = [0; 4];

    for i in 0..4 {
        output[i as usize] = *input
            .get(start_y.checked_add_signed(dy * i)?)?
            .get(start_x.checked_add_signed(dx * i)?)?;
    }

    Some(output)
}

#[test]
fn given_input() {
    assert_eq!(
        wordsearch(
            &b"MMMSXXMASM
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
                .collect::<Vec<_>>()
        ),
        18
    );
}
