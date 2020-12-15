use std::collections::{HashMap, HashSet};

use aoc_runner_derive::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct Colour {
    modifier: String,
    hue: String,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let bag_regex = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    input
        .lines()
        .map(|line| {
            let outer_bag = line[..line.match_indices(' ').nth(1).unwrap().0].to_owned();
            let inner_bags = bag_regex
                .captures_iter(line)
                .map(|bag| (bag[1].parse().unwrap(), bag[2].to_owned()))
                .collect();
            (outer_bag, inner_bags)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn day7_part1(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut reversed: HashMap<_, Vec<_>> = HashMap::new();
    for (outer, inners) in input.clone() {
        for (_, inner) in inners {
            reversed.entry(inner).or_default().push(outer.clone());
        }
    }
    let mut set = HashSet::new();
    let mut added = Vec::new();
    set.extend(&reversed["shiny gold"]);
    added.extend(&reversed["shiny gold"]);
    while let Some(next) = added.pop() {
        if let Some(outers) = reversed.get(next) {
            for outer in outers {
                if set.insert(outer) {
                    added.push(outer);
                }
            }
        }
    }
    set.len()
}

#[aoc(day7, part2)]
pub fn day7_part2(input: &HashMap<String, Vec<(usize, String)>>) -> usize {
    bags_in_bag(input, "shiny gold")
}

fn bags_in_bag(input: &HashMap<String, Vec<(usize, String)>>, bag: &str) -> usize {
    let bags = match input.get(bag) {
        Some(bags) => bags,
        None => return 0,
    };
    bags.iter()
        .map(|(num, bag)| num * (1 + bags_in_bag(input, bag)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "\
        light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
        bright white bags contain 1 shiny gold bag.\n\
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
        faded blue bags contain no other bags.\n\
        dotted black bags contain no other bags.\n\
    ";

    const TEST_INPUT_2: &str = "\
        shiny gold bags contain 2 dark red bags.\n\
        dark red bags contain 2 dark orange bags.\n\
        dark orange bags contain 2 dark yellow bags.\n\
        dark yellow bags contain 2 dark green bags.\n\
        dark green bags contain 2 dark blue bags.\n\
        dark blue bags contain 2 dark violet bags.\n\
        dark violet bags contain no other bags.\n\
    ";

    #[test]
    fn part1() {
        let input = input_generator(TEST_INPUT_1);
        let answer = day7_part1(&input);
        assert_eq!(4, answer);
    }

    #[test]
    fn part2_input1() {
        let input = input_generator(TEST_INPUT_1);
        let answer = day7_part2(&input);
        assert_eq!(32, answer);
    }

    #[test]
    fn part2_input2() {
        let input = input_generator(TEST_INPUT_2);
        let answer = day7_part2(&input);
        assert_eq!(126, answer);
    }
}
