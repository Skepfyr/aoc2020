use std::collections::HashMap;

use aoc_runner_derive::*;

#[derive(Debug, Clone)]
struct MemoryGame {
    time: u64,
    times: HashMap<u64, u64>,
    starting_numbers: Vec<u64>,
    previous: u64,
}

impl MemoryGame {
    fn new(starting_numbers: Vec<u64>) -> Self {
        Self {
            time: 0,
            times: HashMap::new(),
            starting_numbers,
            previous: 0,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_num = (|| {
            if (self.time as usize) < self.starting_numbers.len() {
                return self.starting_numbers[self.time as usize];
            }
            match self.times.get(&self.previous) {
                Some(time) => self.time - time,
                None => 0,
            }
        })();
        self.times.insert(self.previous, self.time);
        self.time += 1;
        self.previous = next_num;
        Some(next_num)
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn day15_part1(input: &[u64]) -> u64 {
    let mut game = MemoryGame::new(input.to_vec());
    game.nth(2020 - 1).unwrap()
}

#[aoc(day15, part2)]
pub fn day15_part2(input: &[u64]) -> u64 {
    let mut game = MemoryGame::new(input.to_vec());
    game.nth(30_000_000 - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_game() {
        let game = MemoryGame::new(vec![0, 3, 6]);
        assert_eq!(
            vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0],
            game.take(10).collect::<Vec<_>>()
        );
        let mut game = MemoryGame::new(vec![0, 3, 6]);
        assert_eq!(436, game.nth(2020 - 1).unwrap());
    }
}
