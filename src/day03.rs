use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MapSquare {
    Open,
    Tree,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Box<[Box<[MapSquare]>]> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => MapSquare::Open,
                    '#' => MapSquare::Tree,
                    _ => panic!("Unexpected character"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn day3_part1(input: &[Box<[MapSquare]>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, row)| row[(i * 3) % row.len()] == MapSquare::Tree)
        .count()
}

#[aoc(day3, part2)]
pub fn day3_part2(input: &[Box<[MapSquare]>]) -> usize {
    const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let trees = input
        .iter()
        .enumerate()
        .fold([0usize; SLOPES.len()], |mut counts, (i, row)| {
            for (count, &(right, down)) in counts.iter_mut().zip(SLOPES.iter()) {
                if i % down == 0 && row[(i * right / down) % row.len()] == MapSquare::Tree {
                    *count += 1;
                }
            }
            counts
        });
    trees.iter().product()
}
