use std::{iter::FromIterator, str::FromStr};

use aoc_runner_derive::*;

#[derive(Debug, Clone)]
pub struct RuleSet {
    pub rules: Vec<Rule>,
}

impl RuleSet {
    pub fn matches(&self, input: &str) -> bool {
        let output = self.matches_rule(input, 0);
        !output.is_empty() && output.iter().any(|s| s.is_empty())
    }

    fn matches_rule<'a>(&self, input: &'a str, rule: usize) -> Vec<&'a str> {
        match &self.rules[rule] {
            Rule::Char(c) => input.strip_prefix(*c).into_iter().collect(),
            Rule::Alternation(alts) => alts
                .iter()
                .flat_map(|sequence| {
                    let mut options = vec![input];
                    for &rule in sequence {
                        options = options
                            .into_iter()
                            .flat_map(|input| self.matches_rule(input, rule))
                            .collect();
                    }
                    options
                })
                .collect(),
        }
    }
}

impl FromIterator<(usize, Rule)> for RuleSet {
    fn from_iter<T: IntoIterator<Item = (usize, Rule)>>(iter: T) -> Self {
        let mut rules = Vec::new();
        for (i, rule) in iter {
            rules.extend(
                std::iter::from_fn(|| Some(Rule::Alternation(Vec::new())))
                    .take((i + 1).saturating_sub(rules.len())),
            );
            rules[i] = rule;
        }
        Self { rules }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Char(char),
    Alternation(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('"') {
            Rule::Char(s.chars().nth(1).ok_or("Single '\"' is not a valid rule.")?)
        } else {
            Rule::Alternation(
                s.split('|')
                    .map(|sequence| {
                        sequence
                            .trim()
                            .split(' ')
                            .map(|num| num.parse::<usize>().map_err(|e| e.to_string()))
                            .collect()
                    })
                    .collect::<Result<_, _>>()?,
            )
        })
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (RuleSet, Vec<String>) {
    let mut lines = input.lines();
    let rule_set = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (index, rule) = line.split_once(':').unwrap();
            (index.parse().unwrap(), rule.trim().parse().unwrap())
        })
        .collect();
    let images = lines.map(ToOwned::to_owned).collect();
    (rule_set, images)
}

#[aoc(day19, part1)]
pub fn day19_part1((rule_set, images): &(RuleSet, Vec<String>)) -> usize {
    images
        .iter()
        .filter(|image| rule_set.matches(image))
        .count()
}

#[aoc(day19, part2)]
pub fn day19_part2((rule_set, images): &(RuleSet, Vec<String>)) -> usize {
    let mut rule_set = rule_set.clone();
    rule_set.rules[8] = Rule::Alternation(vec![vec![42], vec![42, 8]]);
    rule_set.rules[11] = Rule::Alternation(vec![vec![42, 31], vec![42, 11, 31]]);
    images
        .iter()
        .filter(|image| rule_set.matches(image))
        .count()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rule_set = RuleSet {
            rules: vec![
                Rule::Alternation(vec![vec![4, 1, 5]]),
                Rule::Alternation(vec![vec![2, 3], vec![3, 2]]),
                Rule::Alternation(vec![vec![4, 4], vec![5, 5]]),
                Rule::Alternation(vec![vec![4, 5], vec![5, 4]]),
                Rule::Char('a'),
                Rule::Char('b'),
            ],
        };
        assert!(rule_set.matches("aaaabb"));
        assert!(rule_set.matches("aaabab"));
        assert!(rule_set.matches("abbabb"));
        assert!(rule_set.matches("abbbab"));
        assert!(rule_set.matches("aabaab"));
        assert!(rule_set.matches("aabbbb"));
        assert!(rule_set.matches("abaaab"));
        assert!(rule_set.matches("ababbb"));
        assert!(!rule_set.matches("babbba"));
        assert!(!rule_set.matches("aabbab"));
    }
}
