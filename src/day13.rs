use aoc_runner_derive::*;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (i64, Vec<Option<i64>>) {
    let mut lines = input.lines();
    let earliest_time = lines.next().unwrap().parse().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|id| id.parse().ok())
        .collect();
    (earliest_time, bus_ids)
}

#[aoc(day13, part1)]
pub fn day13_part1((time, bus_ids): &(i64, Vec<Option<i64>>)) -> i64 {
    let mut ids: Vec<_> = bus_ids
        .iter()
        .filter_map(|p| *p)
        .map(|id| (id, id - time % id))
        .collect();
    ids.sort_unstable_by_key(|(_, time)| *time);
    let (id, wait_time) = ids[0];
    id * wait_time
}

#[aoc(day13, part2)]
pub fn day13_part2((_, constraints): &(i64, Vec<Option<i64>>)) -> i64 {
    let constraints: Vec<_> = constraints
        .iter()
        .enumerate()
        .filter_map(|(i, &id)| id.map(|id| (i, id)))
        .collect();
    let product: i64 = constraints.iter().map(|&(_, id)| id).product();
    constraints
        .iter()
        .map(|&(offset, id)| {
            let multiplier = product / id;
            let (inv, _) = extended_euclid(multiplier, id);
            multiplier * (inv * -(offset as i64)).rem_euclid(id)
        })
        .sum::<i64>()
        .rem_euclid(product)
}

#[allow(clippy::many_single_char_names)]
fn extended_euclid(a: i64, b: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        939\n\
        7,13,x,x,59,x,31,19\n\
    ";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        let res = day13_part1(&input);
        assert_eq!(295, res);
    }

    #[test]
    fn part2() {
        let res = day13_part2(&(0, vec![Some(17), None, Some(13), Some(19)]));
        assert_eq!(3417, res)
    }

    #[test]
    fn euclid() {
        let res = extended_euclid(240, 46);
        assert_eq!((-9, 47), res)
    }
}
