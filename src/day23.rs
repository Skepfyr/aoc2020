use std::fmt::{self, Write as _};

use aoc_runner_derive::*;

#[derive(Debug, Clone)]
pub struct CupCircle {
    next_cup: Vec<usize>,
    current_cup: usize,
}

impl fmt::Display for CupCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cup = self.current_cup;
        write!(f, "{}", cup)?;
        for _ in 2..self.next_cup.len() {
            cup = self.next_cup[cup];
            write!(f, " {}", cup)?;
        }
        Ok(())
    }
}

impl CupCircle {
    pub fn round(&mut self) {
        let max_label = self.next_cup.len() - 1;
        let next_one = self.next_cup[self.current_cup];
        let next_two = self.next_cup[next_one];
        let next_three = self.next_cup[next_two];
        let mut dest_cup = self.current_cup;
        loop {
            dest_cup -= 1;
            if dest_cup == 0 {
                dest_cup = max_label;
            }
            if dest_cup != next_one && dest_cup != next_two && dest_cup != next_three {
                break;
            }
        }
        self.next_cup[self.current_cup] = self.next_cup[next_three];
        self.next_cup[next_three] = self.next_cup[dest_cup];
        self.next_cup[dest_cup] = next_one;
        self.current_cup = self.next_cup[self.current_cup];
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> CupCircle {
    let mut next_cup = Vec::new();
    for &[cup, next] in input.as_bytes().array_windows() {
        let cup = (cup as char).to_digit(10).unwrap() as usize;
        let next = (next as char).to_digit(10).unwrap() as usize;
        next_cup.extend(std::iter::repeat(0).take((cup + 1).saturating_sub(next_cup.len())));
        next_cup[cup] = next;
    }
    let first = (*input.as_bytes().first().unwrap() as char)
        .to_digit(10)
        .unwrap() as usize;
    let last = (*input.as_bytes().last().unwrap() as char)
        .to_digit(10)
        .unwrap() as usize;
    next_cup[last] = first;
    CupCircle {
        next_cup,
        current_cup: first,
    }
}

#[aoc(day23, part1)]
pub fn day23_part1(input: &CupCircle) -> String {
    let mut circle = input.clone();
    for _ in 0..100 {
        circle.round();
    }
    let mut answer = String::with_capacity(input.next_cup.len());
    let mut cup = 1;
    for _ in 2..input.next_cup.len() {
        cup = circle.next_cup[cup];
        write!(answer, "{}", cup).unwrap();
    }
    answer
}

#[aoc(day23, part2)]
pub fn day23_part2(input: &CupCircle) -> usize {
    let mut circle = input.clone();
    let first = circle.current_cup;
    let last = circle
        .next_cup
        .iter()
        .position(|&cup| cup == first)
        .unwrap();
    let next = circle.next_cup.len();
    circle.next_cup[last] = next;
    circle.next_cup.extend(next + 1..=1_000_000);
    circle.next_cup.push(first);
    for _ in 0..10_000_000 {
        circle.round();
    }
    let next_one = circle.next_cup[1];
    let next_two = circle.next_cup[next_one];
    next_one * next_two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_round() {
        let mut circle = input_generator("389125467");
        for i in 1..=10 {
            println!("-- move {} --", i);
            println!("cups: {}", circle);
            circle.round();
            println!();
        }
        println!("-- final --");
        println!("cups: {}", circle);
    }

    #[test]
    fn part2() {
        let circle = input_generator("389125467");
        assert_eq!(149245887792, day23_part2(&circle));
    }
}
