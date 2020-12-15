use aoc_runner_derive::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Move {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    TurnLeft(i32),
    TurnRight(i32),
    MoveForward(i32),
}

#[derive(Debug)]
pub struct Ship {
    heading: i32,
    position: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship {
    fn new() -> Self {
        Self {
            heading: 90,
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn move_by(&mut self, movement: Move) {
        match movement {
            Move::North(dist) => self.move_heading(0, dist),
            Move::South(dist) => self.move_heading(180, dist),
            Move::East(dist) => self.move_heading(90, dist),
            Move::West(dist) => self.move_heading(270, dist),
            Move::TurnLeft(degrees) => self.heading = (self.heading - degrees).rem_euclid(360),
            Move::TurnRight(degrees) => self.heading = (self.heading + degrees).rem_euclid(360),
            Move::MoveForward(dist) => self.move_heading(self.heading, dist),
        }
    }

    #[allow(clippy::zero_prefixed_literal)]
    fn move_heading(&mut self, heading: i32, dist: i32) {
        match heading {
            000 => self.position.1 += dist,
            090 => self.position.0 += dist,
            180 => self.position.1 -= dist,
            270 => self.position.0 -= dist,
            _ => panic!("Can't move with heading {}", heading),
        }
    }

    fn move_waypoint(&mut self, movement: Move) {
        match movement {
            Move::North(dist) => self.waypoint.1 += dist,
            Move::South(dist) => self.waypoint.1 -= dist,
            Move::East(dist) => self.waypoint.0 += dist,
            Move::West(dist) => self.waypoint.0 -= dist,
            Move::TurnLeft(degrees) => self.rotate_waypoint(360 - degrees),
            Move::TurnRight(degrees) => self.rotate_waypoint(degrees),
            Move::MoveForward(dist) => {
                self.position.0 += dist * self.waypoint.0;
                self.position.1 += dist * self.waypoint.1;
            }
        }
    }

    #[allow(clippy::zero_prefixed_literal)]
    fn rotate_waypoint(&mut self, degrees: i32) {
        self.waypoint = match degrees {
            000 => self.waypoint,
            090 => (self.waypoint.1, -self.waypoint.0),
            180 => (-self.waypoint.0, -self.waypoint.1),
            270 => (-self.waypoint.1, self.waypoint.0),
            _ => panic!("Cannot rotate by {}", degrees),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (c, num) = line.split_at(1);
            let num = num.parse().unwrap();
            match c {
                "N" => Move::North(num),
                "S" => Move::South(num),
                "E" => Move::East(num),
                "W" => Move::West(num),
                "L" => Move::TurnLeft(num),
                "R" => Move::TurnRight(num),
                "F" => Move::MoveForward(num),
                _ => panic!("Unexpected movement command {} for {}", c, num),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn day12_part1(input: &[Move]) -> i32 {
    let mut ship = Ship::new();
    for &movement in input {
        ship.move_by(movement);
    }
    ship.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn day12_part2(input: &[Move]) -> i32 {
    let mut ship = Ship::new();
    for &movement in input {
        ship.move_waypoint(movement);
    }
    ship.manhattan_distance()
}
