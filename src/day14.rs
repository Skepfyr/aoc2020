use std::collections::HashMap;

use aoc_runner_derive::*;

#[derive(Debug, Copy, Clone)]
pub struct BitMask {
    mask: [Option<bool>; 36],
}

impl BitMask {
    fn new(bitmask: &str) -> Self {
        let bitmask = &bitmask[7..];
        assert_eq!(36, bitmask.len());
        let mut mask = [None; 36];
        for (i, &c) in bitmask.as_bytes().iter().rev().enumerate() {
            mask[i] = match c {
                b'0' => Some(false),
                b'1' => Some(true),
                b'X' => None,
                c => panic!("Unexpected character in bitmask: {}", c),
            };
        }
        Self { mask }
    }

    fn mask(self, mut value: u64) -> u64 {
        for (i, &bit_mask) in self.mask.iter().enumerate() {
            if let Some(bit) = bit_mask {
                value = (value & !(1 << i)) | (if bit { 1 } else { 0 } << i);
            }
        }
        value
    }

    fn values(self) -> BitMaskIter {
        BitMaskIter::new(self)
    }
}

impl std::ops::BitOr<u64> for BitMask {
    type Output = BitMask;

    fn bitor(mut self, rhs: u64) -> Self::Output {
        for (i, bit) in self.mask.iter_mut().enumerate() {
            if rhs & (1 << i) != 0 && bit.is_some() {
                *bit = Some(true);
            }
        }
        self
    }
}

#[derive(Debug)]
pub struct BitMaskIter {
    mask: BitMask,
    value: u64,
    finished: bool,
}

impl BitMaskIter {
    fn new(mask: BitMask) -> Self {
        let value = mask.mask.iter().rev().fold(0u64, |acc, &bit| {
            (acc << 1) | if bit == Some(true) { 1 } else { 0 }
        });
        Self {
            mask,
            value,
            finished: false,
        }
    }
}

impl Iterator for BitMaskIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let res = self.value;
        let mut overflow = true;
        for (i, &bit) in self.mask.mask.iter().enumerate() {
            if bit.is_some() {
                continue;
            }
            self.value ^= 1 << i;
            if self.value & (1 << i) != 0 {
                overflow = false;
                break;
            }
        }
        if overflow {
            self.finished = true;
        }
        Some(res)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Assignment {
    address: u64,
    value: u64,
}

impl Assignment {
    fn new(assignment: &str) -> Self {
        let address_start = assignment
            .find('[')
            .unwrap_or_else(|| panic!("Couldn't find address start in: {}", assignment))
            + 1;
        let address_end = assignment
            .find(']')
            .unwrap_or_else(|| panic!("Couldn't find address end in: {}", assignment));
        let value_start = address_end + 4;
        Self {
            address: assignment[address_start..address_end]
                .parse()
                .unwrap_or_else(|_| panic!("Address not a number in: {}", assignment)),
            value: assignment[value_start..]
                .parse()
                .unwrap_or_else(|_| panic!("Value not a number in {}", assignment)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    BitMask(BitMask),
    Assignment(Assignment),
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                Instruction::BitMask(BitMask::new(line))
            } else if line.starts_with("mem") {
                Instruction::Assignment(Assignment::new(line))
            } else {
                panic!("Unexpected input line {}", line);
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn day14_part1(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = BitMask::new("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    for instruction in instructions {
        match instruction {
            Instruction::BitMask(mask) => current_mask = *mask,
            Instruction::Assignment(assignment) => {
                let value = current_mask.mask(assignment.value);
                memory.insert(assignment.address, value);
            }
        }
    }
    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn day14_part2(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = BitMask::new("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    for instruction in instructions {
        match instruction {
            Instruction::BitMask(mask) => current_mask = *mask,
            Instruction::Assignment(assignment) => {
                for address in (current_mask | assignment.address).values() {
                    memory.insert(address, assignment.value);
                }
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask() {
        let mask = BitMask::new("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(73, mask.mask(11));
        assert_eq!(101, mask.mask(101));
        assert_eq!(64, mask.mask(0));
    }

    #[test]
    fn values() {
        let mask = BitMask::new("mask = 000000000000000000000000000000X1001X");
        assert_eq!(
            vec![26, 27, 58, 59],
            (mask | 42).values().collect::<Vec<_>>()
        );
    }
}
