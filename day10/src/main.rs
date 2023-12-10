fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::parse(input, Connection::NorthSouth);
    println!("Part 1: {}", grid.loop_length() / 2);
}

struct Grid {
    starting_position: (usize, usize),
    paths: Vec<Vec<Connection>>,
}

impl Grid {
    fn parse(input: &str, animal_start_connection: Connection) -> Self {
        let mut paths: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.bytes().map(Connection::parse).collect())
            .collect();

        let mut animal_start = (0, 0);

        for (y, row) in paths.iter_mut().enumerate() {
            for (x, connection) in row.iter_mut().enumerate() {
                if *connection == Connection::Animal {
                    animal_start = (x, y);
                    *connection = animal_start_connection;
                }
            }
        }

        Self {
            paths,
            starting_position: animal_start,
        }
    }

    fn loop_length(&self) -> usize {
        let mut length = 0;
        let mut position = self.starting_position;
        let mut previous_position = position;

        loop {
            length += 1;
            let potential_directions = self.paths[position.1][position.0].next_positions(position);

            if potential_directions.0 == previous_position {
                previous_position = position;
                position = potential_directions.1;
            } else {
                previous_position = position;
                position = potential_directions.0;
            }

            if position == self.starting_position {
                break;
            }
        }

        length
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Connection {
    NorthSouth,
    NorthEast,
    NorthWest,
    EastWest,
    EastSouth,
    SouthWest,

    Ground,
    Animal,
}

impl Connection {
    /*
        | is a vertical pipe connecting north and south.
        - is a horizontal pipe connecting east and west.
        L is a 90-degree bend connecting north and east.
        J is a 90-degree bend connecting north and west.
        7 is a 90-degree bend connecting south and west.
        F is a 90-degree bend connecting south and east.
        . is ground; there is no pipe in this tile.
        S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    */
    fn parse(input: u8) -> Connection {
        match input {
            b'|' => Connection::NorthSouth,
            b'-' => Connection::EastWest,
            b'L' => Connection::NorthEast,
            b'J' => Connection::NorthWest,
            b'7' => Connection::SouthWest,
            b'F' => Connection::EastSouth,
            b'.' => Connection::Ground,
            b'S' => Connection::Animal,
            _ => panic!("Unknown character {input}"),
        }
    }

    fn next_positions(self, (x, y): (usize, usize)) -> ((usize, usize), (usize, usize)) {
        match self {
            Connection::NorthSouth => ((x, y + 1), (x, y - 1)),
            Connection::NorthEast => ((x, y - 1), (x + 1, y)),
            Connection::NorthWest => ((x, y - 1), (x - 1, y)),
            Connection::EastWest => ((x - 1, y), (x + 1, y)),
            Connection::EastSouth => ((x + 1, y), (x, y + 1)),
            Connection::SouthWest => ((x - 1, y), (x, y + 1)),
            Connection::Ground | Connection::Animal => panic!("Bad type {self:?}"),
        }
    }
}

#[test]
fn given_input() {
    let text = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    let grid = Grid::parse(text, Connection::EastSouth);
    assert_eq!(grid.loop_length(), 16);
}
