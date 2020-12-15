use std::{fmt::Display, ops::RangeInclusive};

use aoc_runner_derive::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Position {
    Floor,
    Seat,
    Person,
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<Position>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for pos in row {
                f.write_str(match pos {
                    Position::Floor => ".",
                    Position::Seat => "L",
                    Position::Person => "#",
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(grid: Vec<Vec<Position>>) -> Self {
        Self { grid }
    }

    pub fn step(
        &mut self,
        count_fn: impl Fn(&Grid, usize, usize) -> usize,
        birth_range: RangeInclusive<usize>,
        death_range: RangeInclusive<usize>,
    ) -> bool {
        let mut changed = false;
        let old = self.clone();
        for i in 0..old.grid.len() {
            for j in 0..old.grid[i].len() {
                self.grid[i][j] = match (old.grid[i][j], count_fn(&old, i, j)) {
                    (Position::Seat, count) if birth_range.contains(&count) => {
                        changed = true;
                        Position::Person
                    }
                    (Position::Person, count) if death_range.contains(&count) => {
                        changed = true;
                        Position::Seat
                    }
                    (p, _) => p,
                };
            }
        }
        changed
    }

    fn adjacent_people(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for i in row.saturating_sub(1)..=usize::min(row + 1, self.grid.len() - 1) {
            for j in col.saturating_sub(1)..=usize::min(col + 1, self.grid[row].len() - 1) {
                if i == row && j == col {
                    continue;
                }
                if self.grid[i][j] == Position::Person {
                    count += 1;
                }
            }
        }
        count
    }

    fn line_of_sight_adjacent(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for y_dir in -1..=1 {
            for x_dir in -1..=1 {
                if x_dir == 0 && y_dir == 0 {
                    continue;
                }
                let mut i = row as isize;
                let mut j = col as isize;
                loop {
                    i += x_dir;
                    j += y_dir;
                    if !(0..self.grid.len() as isize).contains(&i)
                        || !(0..self.grid[i as usize].len() as isize).contains(&j)
                    {
                        break;
                    }
                    match self.grid[i as usize][j as usize] {
                        Position::Floor => continue,
                        Position::Seat => break,
                        Position::Person => {
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }
        count
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Position::Floor,
                        'L' => Position::Seat,
                        '#' => Position::Person,
                        c => panic!("Unexpected character {}", c),
                    })
                    .collect()
            })
            .collect(),
    )
}

#[aoc(day11, part1)]
pub fn day11_part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    while grid.step(Grid::adjacent_people, 0..=0, 4..=8) {}
    grid.grid
        .iter()
        .flatten()
        .filter(|&&pos| pos == Position::Person)
        .count()
}

#[aoc(day11, part2)]
pub fn day11_part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    while grid.step(Grid::line_of_sight_adjacent, 0..=0, 5..=8) {}
    grid.grid
        .iter()
        .flatten()
        .filter(|&&pos| pos == Position::Person)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL\n\
    ";

    #[test]
    fn part1() {
        let grid = input_generator(INPUT);
        let occupied = day11_part1(&grid);
        assert_eq!(37, occupied);
    }
}
