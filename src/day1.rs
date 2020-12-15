use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &[u32]) -> u32 {
    let mut input = input.to_vec();
    input.sort_unstable();
    let mut start = 0;
    let mut end = input.len() - 1;
    loop {
        if start >= end {
            panic!("No pair sum to 2020.");
        }
        match (unsafe { input.get_unchecked(start) + input.get_unchecked(end) }).cmp(&2020) {
            Ordering::Less => {
                start += 1;
            }
            Ordering::Equal => {
                break input[start] * input[end];
            }
            Ordering::Greater => {
                end -= 1;
            }
        }
    }
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &[u32]) -> u32 {
    let mut input = input.to_vec();
    input.sort_unstable();
    for (a_idx, &a) in input.iter().enumerate() {
        let mut last_c_idx = input.len();
        for &b in input.iter().skip(a_idx + 1) {
            if a + b >= 2020 {
                break;
            }
            match input[..last_c_idx].binary_search(&(2020 - a - b)) {
                Ok(c) => return a * b * input[c],
                Err(c_idx) => last_c_idx = c_idx,
            }
        }
    }
    panic!("No triplet sum to 2020")
}
