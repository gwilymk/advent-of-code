fn main() {
    let part1 = part1(&[(41, 249), (77, 1362), (70, 1127), (96, 1011)]);
    println!("Part 1: {part1}");

    let part2 = ways_to_beat_the_record(41777096, 249136211271011);
    println!("Part 2: {part2}");
}

fn ways_to_beat_the_record(time: usize, record_distance: usize) -> usize {
    let mut number_of_ways = 0;

    for button_time in 0..time {
        let speed = button_time;
        let time_to_move = time - button_time;

        let distance = time_to_move * speed;
        if distance > record_distance {
            number_of_ways += 1;
        }
    }

    number_of_ways
}

fn part1(races: &[(usize, usize)]) -> usize {
    races
        .iter()
        .map(|race| ways_to_beat_the_record(race.0, race.1))
        .product()
}

#[test]
fn test_input_part_1() {
    assert_eq!(ways_to_beat_the_record(7, 9), 4);

    assert_eq!(part1(&[(7, 9), (15, 40), (30, 200)]), 288);
}
