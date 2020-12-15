use std::collections::{BTreeMap, HashMap};

use aoc_runner_derive::*;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn day10_part1(input: &[u64]) -> u64 {
    let mut adapters = vec![0];
    adapters.extend_from_slice(input);
    adapters.sort_unstable();
    let counts = adapters.array_windows().map(|&[a, b]| b - a).fold(
        HashMap::<u64, u64>::new(),
        |mut acc, diff| {
            *acc.entry(diff).or_default() += 1;
            acc
        },
    );
    counts[&1] * (counts[&3] + 1)
}

#[aoc(day10, part2)]
pub fn day10_part2(input: &[u64]) -> u64 {
    let mut adapters = BTreeMap::new();
    adapters.insert(0, 1);
    let mut values = input.to_vec();
    values.sort_unstable();
    for value in values.into_iter() {
        let get = |map: &BTreeMap<u64, u64>, val: u64, offset: u64| -> u64 {
            let key = match val.checked_sub(offset) {
                Some(key) => key,
                None => return 0,
            };
            map.get(&key).copied().unwrap_or(0)
        };
        let prev_1 = get(&adapters, value, 1);
        let prev_2 = get(&adapters, value, 2);
        let prev_3 = get(&adapters, value, 3);
        adapters.insert(value, prev_1 + prev_2 + prev_3);
    }
    adapters.pop_last().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: [u64; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    #[test]
    fn short_input_part1() {
        assert_eq!(35, day10_part1(&INPUT1));
    }

    #[test]
    fn short_input_part2() {
        assert_eq!(8, day10_part2(&INPUT1));
    }
}
