use std::ops::RangeInclusive;

use aoc_runner_derive::*;

#[derive(Debug)]
pub struct Input {
    fields: Vec<(String, Vec<RangeInclusive<u64>>)>,
    ticket: Vec<u64>,
    other_tickets: Vec<Vec<u64>>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let fields = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, ranges) = line.split_once(':').unwrap();
            let ranges = ranges
                .split("or")
                .map(|range| {
                    let range = range.trim();
                    let (start, end) = range.split_once('-').unwrap();
                    start.parse().unwrap()..=end.parse().unwrap()
                })
                .collect();
            (name.to_owned(), ranges)
        })
        .collect();

    assert_eq!("your ticket:", lines.next().unwrap());
    let ticket: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut other_tickets = Vec::new();
    assert_eq!("", lines.next().unwrap());
    assert_eq!("nearby tickets:", lines.next().unwrap());

    other_tickets
        .extend(lines.map(|line| line.split(',').map(|num| num.parse().unwrap()).collect()));

    Input {
        fields,
        ticket,
        other_tickets,
    }
}

#[aoc(day16, part1)]
pub fn day16_part1(input: &Input) -> u64 {
    input
        .other_tickets
        .iter()
        .flatten()
        .copied()
        .filter(|value| {
            !input
                .fields
                .iter()
                .flat_map(|(_, v)| v)
                .any(|range| range.contains(value))
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn day16_part2(input: &Input) -> u64 {
    let valid_tickets =
        input
            .other_tickets
            .iter()
            .map(|vec| vec.as_slice())
            .filter(move |ticket| {
                ticket.iter().all(|value| {
                    input
                        .fields
                        .iter()
                        .flat_map(|(_, v)| v)
                        .any(|range| range.contains(value))
                })
            });
    let mut options: Vec<_> = (0..input.ticket.len())
        .map(|_| vec![true; input.fields.len()])
        .collect();
    for ticket in valid_tickets {
        for (i, value) in ticket.iter().enumerate() {
            for (j, (_, field_ranges)) in input.fields.iter().enumerate() {
                if !field_ranges.iter().any(|range| range.contains(value)) {
                    options[i][j] = false;
                }
            }
        }
    }
    let mut permutation: Vec<Option<usize>> = vec![None; input.ticket.len()];
    for _ in 0..permutation.len() {
        let (loc, field) = options
            .iter()
            .enumerate()
            .find_map(|(loc, possibles)| {
                let mut only_option = None;
                for (field, &possible) in possibles.iter().enumerate() {
                    if possible {
                        if only_option.is_some() {
                            return None;
                        } else {
                            only_option = Some((loc, field));
                        }
                    }
                }
                only_option
            })
            .unwrap();
        for option in options.iter_mut() {
            option[field] = false;
        }
        permutation[field] = Some(loc);
    }
    input
        .fields
        .iter()
        .enumerate()
        .filter(|(_, (field, _))| field.starts_with("departure"))
        .map(|(field, _)| input.ticket[permutation[field].unwrap()])
        .product()
}
