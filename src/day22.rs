use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use aoc_runner_derive::*;

#[derive(Debug, Clone)]
pub struct Combat {
    player1: VecDeque<u32>,
    player2: VecDeque<u32>,
}

#[derive(Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl Combat {
    pub fn score(&self, player: Player) -> u32 {
        let player = match player {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        };
        player
            .iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (i as u32 + 1) * card)
            .sum()
    }

    pub fn round(&mut self) -> Option<Player> {
        let player1 = match self.player1.pop_front() {
            Some(player1) => player1,
            None => return Some(Player::Player2),
        };
        let player2 = match self.player2.pop_front() {
            Some(player2) => player2,
            None => return Some(Player::Player1),
        };
        if player1 > player2 {
            self.player1.push_back(player1);
            self.player1.push_back(player2);
        } else {
            self.player2.push_back(player2);
            self.player2.push_back(player1);
        }
        None
    }

    pub fn recursive_game(&mut self) -> Player {
        let mut previous_rounds = HashSet::new();
        loop {
            if !previous_rounds.insert((self.player1.clone(), self.player2.clone())) {
                return Player::Player1;
            }
            let player1 = match self.player1.pop_front() {
                Some(player1) => player1,
                None => return Player::Player2,
            };
            let player2 = match self.player2.pop_front() {
                Some(player2) => player2,
                None => return Player::Player1,
            };
            let winner =
                if player1 <= self.player1.len() as u32 && player2 <= self.player2.len() as u32 {
                    let mut sub_game = Combat {
                        player1: self
                            .player1
                            .iter()
                            .take(player1 as usize)
                            .copied()
                            .collect(),
                        player2: self
                            .player2
                            .iter()
                            .take(player2 as usize)
                            .copied()
                            .collect(),
                    };
                    sub_game.recursive_game()
                } else if player1 > player2 {
                    Player::Player1
                } else {
                    Player::Player2
                };
            match winner {
                Player::Player1 => {
                    self.player1.push_back(player1);
                    self.player1.push_back(player2);
                }
                Player::Player2 => {
                    self.player2.push_back(player2);
                    self.player2.push_back(player1);
                }
            }
        }
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Combat {
    let (player1, player2) = input.split_once("\n\n").unwrap();
    let player1 = player1
        .trim()
        .lines()
        .skip(1)
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let player2 = player2
        .trim()
        .lines()
        .skip(1)
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    Combat { player1, player2 }
}

#[aoc(day22, part1)]
pub fn day22_part1(input: &Combat) -> u32 {
    let mut combat = input.clone();
    let winner = loop {
        if let Some(winner) = combat.round() {
            break winner;
        }
    };
    combat.score(winner)
}

#[aoc(day22, part2)]
pub fn day22_part2(input: &Combat) -> u32 {
    let mut combat = input.clone();
    let winner = combat.recursive_game();
    combat.score(winner)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Player 1:\n\
        9\n\
        2\n\
        6\n\
        3\n\
        1\n\
        \n\
        Player 2:\n\
        5\n\
        8\n\
        4\n\
        7\n\
        10\n\
    ";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(306, day22_part1(&input));
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(291, day22_part2(&input));
    }
}
