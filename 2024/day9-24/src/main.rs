fn main() {
    let mut disk = part1::Disk::parse(include_str!("input.txt"));

    disk.compact();
    println!("Part 1: {}", disk.checksum());
}

mod part2 {
    use std::fmt::Debug;

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
            let mut current = 0;
            while current < self.blocks.len() {
                let DiskBlock::Free { len } = self.blocks[current] else {
                    current += 1;
                    continue;
                };

                if len == 0 {
                    current += 1;
                    continue;
                }

                // find the last file that will fit here
                let Some(mut file_index) =
                    self.blocks
                        .iter()
                        .enumerate()
                        .rev()
                        .position(|(index, block)| {
                            if index < current {
                                return false;
                            }

                            let DiskBlock::File { len: file_len, .. } = block else {
                                return false;
                            };

                            *file_len <= len
                        })
                else {
                    current += 1;
                    continue;
                };

                file_index = self.blocks.len() - file_index - 1;

                let DiskBlock::File {
                    id: file_id,
                    len: file_len,
                } = self.blocks[file_index]
                else {
                    panic!("Just found this")
                };

                // need to mark the old space as free, just before and just after will be empty
                let DiskBlock::Free { len: before_len } = self.blocks[file_index - 1] else {
                    panic!("should alternate free and full");
                };

                let DiskBlock::Free { len: after_len } = self.blocks[file_index + 1] else {
                    panic!("should alternate free and full")
                };

                let new_free = DiskBlock::Free {
                    len: before_len + file_len + after_len,
                };
                self.blocks.remove(file_index + 1);
                self.blocks.remove(file_index);
                self.blocks[file_index - 1] = new_free;

                self.blocks[current] = DiskBlock::Free { len: 0 };
                self.blocks.insert(
                    current + 1,
                    DiskBlock::File {
                        id: file_id,
                        len: file_len,
                    },
                );
                self.blocks.insert(
                    current + 2,
                    DiskBlock::Free {
                        len: len - file_len,
                    },
                );

                println!("{self:?}");
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
