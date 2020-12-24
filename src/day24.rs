use std::{
    collections::HashSet,
    ops::Add,
};

use aoc_runner_derive::*;

pub type Instruction = Vec<Direction>;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexPos {
    pub northeast: i32,
    pub east: i32,
}

impl HexPos {
    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        Direction::all().into_iter().map(move |dir| self + dir)
    }
}

impl Add<Direction> for HexPos {
    type Output = HexPos;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::East => HexPos {
                northeast: self.northeast,
                east: self.east + 1,
            },
            Direction::SouthEast => HexPos {
                northeast: self.northeast - 1,
                east: self.east + 1,
            },
            Direction::SouthWest => HexPos {
                northeast: self.northeast - 1,
                east: self.east,
            },
            Direction::West => HexPos {
                northeast: self.northeast,
                east: self.east - 1,
            },
            Direction::NorthWest => HexPos {
                northeast: self.northeast + 1,
                east: self.east - 1,
            },
            Direction::NorthEast => HexPos {
                northeast: self.northeast + 1,
                east: self.east,
            },
        }
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let mut directions = Vec::new();
            while let Some(c) = chars.next() {
                let dir = match c {
                    'e' => Direction::East,
                    'w' => Direction::West,
                    'n' => match chars.next() {
                        Some('e') => Direction::NorthEast,
                        Some('w') => Direction::NorthWest,
                        _ => panic!("Unexpected character after 'n'"),
                    },
                    's' => match chars.next() {
                        Some('e') => Direction::SouthEast,
                        Some('w') => Direction::SouthWest,
                        _ => panic!("Unexpected character after 's'"),
                    },
                    _ => panic!("Unexpected character"),
                };
                directions.push(dir);
            }
            directions
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn day24_part1(input: &[Instruction]) -> usize {
    let mut flipped_tiles = HashSet::new();
    for instruction in input {
        let pos = instruction
            .iter()
            .copied()
            .fold(HexPos::default(), |pos, dir| pos + dir);
        if flipped_tiles.contains(&pos) {
            flipped_tiles.remove(&pos);
        } else {
            flipped_tiles.insert(pos);
        }
    }
    flipped_tiles.len()
}

#[aoc(day24, part2)]
pub fn day24_part2(input: &[Instruction]) -> usize {
    let mut flipped_tiles = HashSet::new();
    for instruction in input {
        let pos = instruction
            .iter()
            .copied()
            .fold(HexPos::default(), |pos, dir| pos + dir);
        if flipped_tiles.contains(&pos) {
            flipped_tiles.remove(&pos);
        } else {
            flipped_tiles.insert(pos);
        }
    }
    for _ in 1..=100 {
        let old_tiles = flipped_tiles.clone();
        for &black_tile in &old_tiles {
            let mut neighbour_count = 0;
            for neighbour in black_tile.neighbours() {
                if old_tiles.contains(&neighbour) {
                    neighbour_count += 1;
                } else {
                    let black_neighbours = neighbour
                        .neighbours()
                        .filter(|pos| old_tiles.contains(pos))
                        .count();
                    if black_neighbours == 2 {
                        flipped_tiles.insert(neighbour);
                    }
                }
            }
            if neighbour_count == 0 || neighbour_count > 2 {
                flipped_tiles.remove(&black_tile);
            }
        }
    }
    flipped_tiles.len()
}
