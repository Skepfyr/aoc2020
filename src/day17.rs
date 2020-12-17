use std::{collections::HashSet, fmt, ops::Range};

use aoc_runner_derive::*;
#[derive(Default, Clone)]
pub struct ConwayCube {
    grid: HashSet<(i32, i32, i32)>,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
}

impl fmt::Debug for ConwayCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for z in self.z_range.clone() {
            writeln!(f, "z={}", z)?;
            for y in self.y_range.clone().rev() {
                for x in self.x_range.clone() {
                    if self.grid.contains(&(x, y, z)) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl ConwayCube {
    fn new(grid: &Vec<Vec<bool>>) -> Self {
        let mut cube = Self::default();
        for (y, row) in grid.iter().rev().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                cube.set((x as i32, y as i32, 0), *cell);
            }
        }
        cube
    }

    fn set(&mut self, pos: (i32, i32, i32), active: bool) {
        if active {
            self.grid.insert(pos);
            let expand_range = |range: &mut Range<i32>, value: i32| {
                if range.len() == 0 {
                    *range = value..(value + 1);
                } else if value < range.start {
                    range.start = value;
                } else if value >= range.end {
                    range.end = value + 1
                }
            };
            expand_range(&mut self.x_range, pos.0);
            expand_range(&mut self.y_range, pos.1);
            expand_range(&mut self.z_range, pos.2);
        } else {
            self.grid.remove(&pos);
        }
    }

    fn step(&self) -> Self {
        let mut new = self.clone();
        for x in self.x_range.start - 1..self.x_range.end + 1 {
            for y in self.y_range.start - 1..self.y_range.end + 1 {
                for z in self.z_range.start - 1..self.z_range.end + 1 {
                    let mut count = 0;
                    for x_offset in -1..=1 {
                        for y_offset in -1..=1 {
                            for z_offset in -1..=1 {
                                if x_offset == 0 && y_offset == 0 && z_offset == 0 {
                                    continue;
                                }
                                if self
                                    .grid
                                    .contains(&(x + x_offset, y + y_offset, z + z_offset))
                                {
                                    count += 1;
                                }
                            }
                        }
                    }
                    let activity = self.grid.contains(&(x, y, z));
                    match (activity, count) {
                        (true, 2) | (true, 3) => {}
                        (true, _) => new.set((x, y, z), false),
                        (false, 3) => new.set((x, y, z), true),
                        (false, _) => {}
                    }
                }
            }
        }
        new
    }
}

#[derive(Default, Clone)]
pub struct ConwayCube4d {
    grid: HashSet<(i32, i32, i32, i32)>,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
    w_range: Range<i32>,
}

impl fmt::Debug for ConwayCube4d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for w in self.w_range.clone() {
            for z in self.z_range.clone() {
                writeln!(f, "z={}, w={}", z, w)?;
                for y in self.y_range.clone().rev() {
                    for x in self.x_range.clone() {
                        if self.grid.contains(&(x, y, z, w)) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

impl ConwayCube4d {
    fn new(grid: &Vec<Vec<bool>>) -> Self {
        let mut cube = Self::default();
        for (y, row) in grid.iter().rev().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                cube.set((x as i32, y as i32, 0, 0), *cell);
            }
        }
        cube
    }

    fn set(&mut self, pos: (i32, i32, i32, i32), active: bool) {
        if active {
            self.grid.insert(pos);
            let expand_range = |range: &mut Range<i32>, value: i32| {
                if range.len() == 0 {
                    *range = value..(value + 1);
                } else if value < range.start {
                    range.start = value;
                } else if value >= range.end {
                    range.end = value + 1
                }
            };
            expand_range(&mut self.x_range, pos.0);
            expand_range(&mut self.y_range, pos.1);
            expand_range(&mut self.z_range, pos.2);
            expand_range(&mut self.w_range, pos.3);
        } else {
            self.grid.remove(&pos);
        }
    }

    fn step(&self) -> Self {
        let mut new = self.clone();
        for x in self.x_range.start - 1..self.x_range.end + 1 {
            for y in self.y_range.start - 1..self.y_range.end + 1 {
                for z in self.z_range.start - 1..self.z_range.end + 1 {
                    for w in self.w_range.start - 1..self.w_range.end + 1 {
                        let mut count = 0;
                        for x_offset in -1..=1 {
                            for y_offset in -1..=1 {
                                for z_offset in -1..=1 {
                                    for w_offset in -1..=1 {
                                        if x_offset == 0
                                            && y_offset == 0
                                            && z_offset == 0
                                            && w_offset == 0
                                        {
                                            continue;
                                        }
                                        if self.grid.contains(&(
                                            x + x_offset,
                                            y + y_offset,
                                            z + z_offset,
                                            w + w_offset,
                                        )) {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                        let activity = self.grid.contains(&(x, y, z, w));
                        match (activity, count) {
                            (true, 2) | (true, 3) => {}
                            (true, _) => new.set((x, y, z, w), false),
                            (false, 3) => new.set((x, y, z, w), true),
                            (false, _) => {}
                        }
                    }
                }
            }
        }
        new
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    c => panic!("Unexpected character {:?}", c),
                })
                .collect()
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn day17_part1(input: &Vec<Vec<bool>>) -> usize {
    let mut cube = ConwayCube::new(&input);
    for _ in 0..6 {
        cube = cube.step();
    }
    cube.grid.len()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &Vec<Vec<bool>>) -> usize {
    let mut cube = ConwayCube4d::new(&input);
    for _ in 0..6 {
        cube = cube.step();
    }
    cube.grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glider() {
        let cube = ConwayCube::new(&vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ]);
        println!("{:?}", cube);
        println!("{:?}", cube.step());
        println!("{:?}", cube.step().step());
        assert!(false)
    }
}
