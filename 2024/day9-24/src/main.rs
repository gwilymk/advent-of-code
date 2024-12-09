fn main() {
    let mut disk = part1::Disk::parse(include_str!("input.txt"));

    disk.compact();
    println!("Part 1: {}", disk.checksum());
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
