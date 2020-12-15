use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct Passport {
    birth_year: Option<String>,
    issuer_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

#[derive(Debug, Copy, Clone)]
enum EyeColour {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "amb" => EyeColour::Amber,
            "blu" => EyeColour::Blue,
            "brn" => EyeColour::Brown,
            "gry" => EyeColour::Grey,
            "grn" => EyeColour::Green,
            "hzl" => EyeColour::Hazel,
            "oth" => EyeColour::Other,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum Height {
    Centimeters(u8),
    Inches(u8),
}

impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cm) = s.strip_suffix("cm") {
            cm.parse().map(Height::Centimeters).map_err(|_| ())
        } else if let Some(inches) = s.strip_suffix("in") {
            inches.parse().map(Height::Inches).map_err(|_| ())
        } else {
            Err(())
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport_data| {
            let mut passport = Passport::default();
            for item in passport_data.split_whitespace() {
                let mut parts = item.split(':');
                let field_name = parts.next().unwrap();
                let field_value = parts.next().unwrap();
                assert!(parts.next().is_none());
                let old = match field_name {
                    "byr" => &mut passport.birth_year,
                    "iyr" => &mut passport.issuer_year,
                    "eyr" => &mut passport.expiration_year,
                    "hgt" => &mut passport.height,
                    "hcl" => &mut passport.hair_colour,
                    "ecl" => &mut passport.eye_colour,
                    "pid" => &mut passport.passport_id,
                    "cid" => &mut passport.country_id,
                    field => panic!("Unknown field {}", field),
                }
                .replace(field_value.to_owned());
                assert!(old.is_none());
            }
            passport
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn day4_part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|&pp| {
            pp.birth_year.is_some()
                && pp.issuer_year.is_some()
                && pp.expiration_year.is_some()
                && pp.height.is_some()
                && pp.hair_colour.is_some()
                && pp.eye_colour.is_some()
                && pp.passport_id.is_some()
        })
        .count()
}

#[aoc(day4, part2)]
pub fn day4_part2(input: &[Passport]) -> usize {
    let hair_colour_regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    let passport_regex = Regex::new("^[0-9]{9}$").unwrap();
    input
        .iter()
        .filter(|&pp| {
            let byr_valid = pp
                .birth_year
                .as_ref()
                .and_then(|byr| byr.parse().ok())
                .map(|byr| (1920u16..=2002).contains(&byr))
                .unwrap_or(false);
            let iyr_valid = pp
                .issuer_year
                .as_ref()
                .and_then(|iyr| iyr.parse().ok())
                .map(|iyr| (2010u16..=2020).contains(&iyr))
                .unwrap_or(false);
            let eyr_valid = pp
                .expiration_year
                .as_ref()
                .and_then(|eyr| eyr.parse().ok())
                .map(|eyr| (2020u16..=2030).contains(&eyr))
                .unwrap_or(false);
            let hgt_valid = pp
                .height
                .as_ref()
                .and_then(|hgt| hgt.parse::<Height>().ok())
                .map(|hgt| match hgt {
                    Height::Centimeters(cm) => (150..=193).contains(&cm),
                    Height::Inches(inches) => (59..=76).contains(&inches),
                })
                .unwrap_or(false);
            let hcl_valid = pp
                .hair_colour
                .as_ref()
                .map(|hcl| hair_colour_regex.is_match(hcl))
                .unwrap_or(false);
            let ecl_valid = pp
                .eye_colour
                .as_ref()
                .map(|ecl| ecl.parse::<EyeColour>().ok().is_some())
                .unwrap_or(false);
            let pid_valid = pp
                .passport_id
                .as_ref()
                .map(|pid| passport_regex.is_match(pid))
                .unwrap_or(false);
            byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
        })
        .count()
}
