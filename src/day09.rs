use std::cmp::Ordering;

use aoc_runner_derive::*;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn day9_part1(input: &[u64]) -> u64 {
    input
        .windows(26)
        .find_map(|numbers| {
            let target = numbers[25];
            for &a in &numbers[..25] {
                for &b in &numbers[..25] {
                    if a + b == target {
                        return None;
                    }
                }
            }
            Some(target)
        })
        .unwrap()
}

#[aoc(day9, part2)]
pub fn day9_part2(input: &[u64]) -> u64 {
    let target = day9_part1(input);
    let mut start = 0;
    let mut end = 1;
    let mut sum = input[0];
    loop {
        match sum.cmp(&target) {
            Ordering::Less => {
                sum += input[end];
                end += 1;
            }
            Ordering::Equal => {
                return input[start..end].iter().copied().min().unwrap()
                    + input[start..end].iter().copied().max().unwrap()
            }
            Ordering::Greater => {
                sum -= input[start];
                start += 1;
            }
        }
    }
}
