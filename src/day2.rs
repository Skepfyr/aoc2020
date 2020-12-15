use aoc_runner_derive::{aoc, aoc_generator};

pub struct Password {
    policy_range: (usize, usize),
    policy_char: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let mut policy_range = split.next().unwrap().split('-');
            let policy_range = (
                policy_range.next().unwrap().parse().unwrap(),
                policy_range.next().unwrap().parse().unwrap(),
            );
            let policy_char = split.next().unwrap().chars().next().unwrap();
            let password = split.next().unwrap().trim().to_owned();
            Password {
                policy_range,
                policy_char,
                password,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn day2_part1(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|&password| {
            (password.policy_range.0..=password.policy_range.1).contains(
                &password
                    .password
                    .chars()
                    .filter(|&c| c == password.policy_char)
                    .count(),
            )
        })
        .count()
}

#[aoc(day2, part2)]
pub fn day2_part2(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|&password| {
            let first_char = password
                .password
                .chars()
                .nth(password.policy_range.0 - 1)
                .unwrap();
            let second_char = password
                .password
                .chars()
                .nth(password.policy_range.1 - 1)
                .unwrap();
            (first_char == password.policy_char) ^ (second_char == password.policy_char)
        })
        .count()
}
