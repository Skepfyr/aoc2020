use aoc_runner_derive::*;

#[derive(Debug, Clone, Copy)]
pub struct Keys {
    card: u64,
    door: u64,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Keys {
    let (card, door) = input.split_once('\n').unwrap();
    Keys {
        card: card.trim().parse().unwrap(),
        door: door.trim().parse().unwrap(),
    }
}

#[aoc(day25, part1)]
pub fn day25_part1(&input: &Keys) -> u64 {
    let mut keys = std::iter::successors(Some(1), |key| Some((key * 7) % 20201227)).enumerate();
    let (card_loop_size, _) = keys.clone().find(|&(_, key)| key == input.card).unwrap();
    let (door_loop_size, _) = keys.clone().find(|&(_, key)| key == input.door).unwrap();

    let mut encryption_key = 1;
    for _ in 0..door_loop_size {
        encryption_key = (encryption_key * input.card) % 20201227;
    }
    encryption_key
}
