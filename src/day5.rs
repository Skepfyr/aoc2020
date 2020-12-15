use aoc_runner_derive::*;

#[derive(Debug, Copy, Clone)]
pub struct SeatId {
    row: u8,
    col: u8,
}

impl SeatId {
    pub fn id(&self) -> u16 {
        self.row as u16 * 8 + self.col as u16
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<SeatId> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let row = chars.by_ref().take(7).fold(0, |acc, c| {
                (acc << 1)
                    + match c {
                        'F' => 0,
                        'B' => 1,
                        _ => panic!(),
                    }
            });

            let col = chars.by_ref().take(3).fold(0, |acc, c| {
                (acc << 1)
                    + match c {
                        'L' => 0,
                        'R' => 1,
                        _ => panic!(),
                    }
            });
            SeatId { row, col }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn day5_part1(input: &[SeatId]) -> u16 {
    input.iter().map(SeatId::id).max().unwrap()
}

#[aoc(day5, part2)]
pub fn day5_part2(input: &[SeatId]) -> u16 {
    let mut ids: Vec<_> = input.iter().map(SeatId::id).collect();
    ids.sort_unstable();
    ids.array_windows()
        .find_map(|&[a, b]| (a != b - 1).then_some(a + 1))
        .unwrap()
}
