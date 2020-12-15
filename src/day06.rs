use aoc_runner_derive::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct Answers(pub [bool; 26]);

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<Answers>> {
    let mut groups = Vec::new();
    let mut group_answers = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            groups.push(group_answers);
            group_answers = Vec::new();
        } else {
            let mut answers = Answers::default();
            for c in line.chars() {
                match c {
                    'a'..='z' => answers.0[c as usize - 'a' as usize] = true,
                    _ => panic!("Unexpected answer: {:?}", c),
                }
            }
            group_answers.push(answers);
        }
    }
    groups.push(group_answers);
    groups
}

#[aoc(day6, part1)]
pub fn day6_part1(input: &[Vec<Answers>]) -> usize {
    input
        .iter()
        .map(|group_answers| {
            let mut answers = Answers::default();
            for individual_answers in group_answers {
                for (answer, individual_answer) in answers.0.iter_mut().zip(&individual_answers.0) {
                    *answer |= individual_answer;
                }
            }
            answers.0.iter().filter(|&&b| b).count()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn day6_part2(input: &[Vec<Answers>]) -> usize {
    input
        .iter()
        .map(|group_answers| {
            let mut answers = Answers([true; 26]);
            for individual_answers in group_answers {
                for (answer, individual_answer) in answers.0.iter_mut().zip(&individual_answers.0) {
                    *answer &= individual_answer;
                }
            }
            answers.0.iter().filter(|&&b| b).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n";
        let parsed = input_generator(input);
        let answer = day6_part1(&parsed);
        assert_eq!(11, answer)
    }

    #[test]
    fn part2() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n";
        let parsed = input_generator(input);
        let answer = day6_part2(&parsed);
        assert_eq!(6, answer)
    }
}
