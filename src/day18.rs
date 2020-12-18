use std::iter::Peekable;

use aoc_runner_derive::*;

#[derive(Debug)]
pub enum Expr {
    Sum(Box<Expr>, Box<Expr>),
    Product(Box<Expr>, Box<Expr>),
    Number(i64),
}

impl Expr {
    pub fn parse(s: &str, precedence: &impl Fn(char) -> Option<(u8, u8)>) -> Result<Self, String> {
        let mut chars = s.chars().filter(|c| !c.is_ascii_whitespace()).peekable();
        Self::parse_expr(&mut chars, 0, precedence)
    }

    fn parse_expr(
        chars: &mut Peekable<impl Iterator<Item = char>>,
        min_binding_power: u8,
        precedence: &impl Fn(char) -> Option<(u8, u8)>,
    ) -> Result<Expr, String> {
        let lhs = chars
            .next()
            .ok_or_else(|| "Expected expression, found an empty string".to_owned())?;
        let mut lhs = match lhs {
            '(' => {
                let lhs = Self::parse_expr(chars, 0, precedence)?;
                match chars.next() {
                    Some(')') => {}
                    Some(c) => return Err(format!("Expected ')', got {:?}", c)),
                    None => return Err("Expected ')', got end of input".to_owned()),
                }
                lhs
            }
            digit if digit.is_digit(10) => Expr::Number(digit.to_digit(10).unwrap().into()),
            c => {
                return Err(format!(
                    "Unexpected token at start of expression, got {:?}",
                    c
                ))
            }
        };

        loop {
            let op = match chars.peek() {
                Some(c @ '+') | Some(c @ '*') => *c,
                Some(')') => break,
                Some(c) => return Err(format!("Unexpected token, {:?}", c)),
                None => break,
            };
            let (left_binding_power, right_binding_power) =
                precedence(op).ok_or_else(|| format!("Unexpected operator {:?}", op))?;
            if left_binding_power < min_binding_power {
                break;
            }

            chars.next();
            let rhs = Self::parse_expr(chars, right_binding_power, precedence)?;

            lhs = match op {
                '+' => Expr::Sum(Box::new(lhs), Box::new(rhs)),
                '*' => Expr::Product(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            };
        }

        Ok(lhs)
    }

    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::Sum(a, b) => a.evaluate() + b.evaluate(),
            Expr::Product(a, b) => a.evaluate() * b.evaluate(),
            Expr::Number(n) => *n,
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(ToOwned::to_owned).collect()
}

#[aoc(day18, part1)]
pub fn day18_part1(input: &[String]) -> i64 {
    let precedence = |op| {
        Some(match op {
            '+' | '*' => (1, 2),
            _ => return None,
        })
    };

    input
        .iter()
        .map(|s| Expr::parse(s, &precedence).unwrap().evaluate())
        .sum()
}

#[aoc(day18, part2)]
pub fn day18_part2(input: &[String]) -> i64 {
    let precedence = |op| {
        Some(match op {
            '+' => (3, 4),
            '*' => (1, 2),
            _ => return None,
        })
    };

    input
        .iter()
        .map(|s| Expr::parse(s, &precedence).unwrap().evaluate())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let precedence = |op| {
            Some(match op {
                '+' | '*' => (1, 2),
                _ => return None,
            })
        };
        let expr = Expr::parse("1 + 2 * 3 + 4 * 5 + 6", &precedence).unwrap();
        assert_eq!(71, expr.evaluate())
    }
}
