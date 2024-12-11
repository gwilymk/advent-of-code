use aoc2024::get_input;

fn main() {
    let input = get_input(9);
    let mut disk = part1::Disk::parse(&input);

    disk.compact();
    println!("Part 1: {}", disk.checksum());

    let mut disk2 = part2::Disk::parse(&input);
    disk2.deframent();
    println!("Part 2: {}", disk2.checksum());
}

mod part2 {
    use std::{collections::HashSet, fmt::Debug};

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum DiskBlock {
        File { id: usize, len: usize },
        Free { len: usize },
    }

    pub struct Disk {
        blocks: Vec<DiskBlock>,
    }

    impl Disk {
        pub fn parse(input: &str) -> Self {
            let mut blocks = vec![];

            for (i, c) in input.chars().enumerate() {
                if i % 2 == 0 {
                    blocks.push(DiskBlock::File {
                        id: i / 2,
                        len: c.to_digit(10).unwrap() as usize,
                    });
                } else {
                    blocks.push(DiskBlock::Free {
                        len: c.to_digit(10).unwrap() as usize,
                    });
                }
            }

            blocks.push(DiskBlock::Free { len: 0 });

            Self { blocks }
        }

        pub fn deframent(&mut self) {
            let mut searched_indexes: HashSet<usize> = HashSet::new();

            let mut file_index = self.blocks.len() - 1;

            while file_index > 0 {
                let DiskBlock::File { id, len } = self.blocks[file_index] else {
                    file_index -= 1;
                    continue;
                };

                if searched_indexes.contains(&id) {
                    file_index -= 1;
                    continue;
                }

                searched_indexes.insert(id);

                let Some(free_index) =
                    self.blocks
                        .iter()
                        .take(file_index)
                        .position(|block| match block {
                            DiskBlock::File { .. } => false,
                            DiskBlock::Free { len: free_len } => *free_len >= len,
                        })
                else {
                    file_index -= 1;
                    continue;
                };

                let DiskBlock::Free { len: free_len } = self.blocks[free_index] else {
                    panic!("just found this");
                };

                if free_len == len {
                    self.blocks.swap(free_index, file_index);
                    file_index -= 1;
                } else {
                    self.blocks[file_index] = DiskBlock::Free { len };
                    self.blocks[free_index] = DiskBlock::File { id, len };
                    self.blocks.insert(
                        free_index + 1,
                        DiskBlock::Free {
                            len: free_len - len,
                        },
                    );
                }
            }
        }

        pub fn checksum(&self) -> usize {
            let mut current_id = 0;
            let mut current_checksum = 0;
            for &i in &self.blocks {
                match i {
                    DiskBlock::File { id, len } => {
                        for _ in 0..len {
                            current_checksum += id * current_id;
                            current_id += 1;
                        }
                    }
                    DiskBlock::Free { len } => {
                        current_id += len;
                    }
                }
            }

            current_checksum
        }
    }

    impl Debug for Disk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for block in &self.blocks {
                match block {
                    DiskBlock::File { id, len } => {
                        for _ in 0..*len {
                            write!(f, "{id}")?;
                        }
                    }
                    DiskBlock::Free { len } => {
                        for _ in 0..*len {
                            write!(f, ".")?;
                        }
                    }
                }
            }

            Ok(())
        }
    }

    #[test]
    fn part2_test() {
        let mut disk = Disk::parse("2333133121414131402");

        disk.deframent();

        assert_eq!(disk.checksum(), 2858);
    }
}

mod part1 {
    use std::fmt::Debug;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum DiskBlock {
        File(usize),
        Free,
    }

    pub struct Disk {
        blocks: Vec<DiskBlock>,
    }

    impl Disk {
        pub fn parse(input: &str) -> Self {
            let mut blocks = vec![];

            for (i, c) in input.chars().enumerate() {
                if i % 2 == 0 {
                    blocks.extend(
                        vec![DiskBlock::File(i / 2); c.to_digit(10).unwrap() as usize].iter(),
                    );
                } else {
                    blocks.extend(vec![DiskBlock::Free; c.to_digit(10).unwrap() as usize].iter());
                }
            }

            Self { blocks }
        }

        pub fn compact(&mut self) {
            let mut i = 0;
            while i < self.blocks.len() {
                if self.blocks[i] == DiskBlock::Free {
                    self.blocks.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }

        pub fn checksum(&self) -> usize {
            self.blocks
                .iter()
                .enumerate()
                .map(|(i, b)| match b {
                    DiskBlock::File(id) => i * id,
                    DiskBlock::Free => 0,
                })
                .sum()
        }
    }

    impl Debug for Disk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for block in &self.blocks {
                match block {
                    DiskBlock::File(i) => write!(f, "{i}")?,
                    DiskBlock::Free => write!(f, ".")?,
                }
            }

            Ok(())
        }
    }

    #[test]
    fn given_input() {
        let mut disk = Disk::parse("2333133121414131402");

        disk.compact();

        assert_eq!(disk.checksum(), 1928);
    }
}
