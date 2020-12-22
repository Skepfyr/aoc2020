use std::collections::{BTreeMap, HashMap, HashSet};

use aoc_runner_derive::*;

#[derive(Debug)]
pub struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<Recipe> {
    input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.split_once("(contains").unwrap();
            let ingredients = ingredients
                .trim()
                .split(' ')
                .map(ToOwned::to_owned)
                .collect();
            let allergens = allergens
                .trim()
                .strip_suffix(')')
                .unwrap()
                .split(", ")
                .map(ToOwned::to_owned)
                .collect();
            Recipe {
                ingredients,
                allergens,
            }
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn day21_part1(input: &[Recipe]) -> usize {
    let mut allergen_potentials: HashMap<String, HashSet<String>> = HashMap::new();
    for recipe in input {
        for allergen in &recipe.allergens {
            allergen_potentials
                .entry(allergen.clone())
                .and_modify(|set| {
                    *set = set.intersection(&recipe.ingredients).cloned().collect();
                })
                .or_insert_with(|| recipe.ingredients.clone());
        }
    }
    let maybe_allergenic = allergen_potentials
        .values()
        .fold(HashSet::new(), |acc, ingredients| {
            acc.union(ingredients).cloned().collect()
        });
    input
        .iter()
        .flat_map(|recipe| &recipe.ingredients)
        .filter(|ingredient| !maybe_allergenic.contains(ingredient.as_str()))
        .count()
}

#[aoc(day21, part2)]
pub fn day21_part2(input: &[Recipe]) -> String {
    let mut allergen_potentials: HashMap<String, HashSet<String>> = HashMap::new();
    for recipe in input {
        for allergen in &recipe.allergens {
            allergen_potentials
                .entry(allergen.clone())
                .and_modify(|set| {
                    *set = set.intersection(&recipe.ingredients).cloned().collect();
                })
                .or_insert_with(|| recipe.ingredients.clone());
        }
    }
    let mut allergens = BTreeMap::new();
    for _ in 0..allergen_potentials.len() {
        let (allergen, ingredient) = allergen_potentials
            .iter()
            .find_map(|(k, ingredients)| {
                if ingredients.len() == 1 {
                    Some((k.clone(), ingredients.iter().next().unwrap().clone()))
                } else {
                    None
                }
            })
            .unwrap();
        for ingredients in allergen_potentials.values_mut() {
            ingredients.remove(&ingredient);
        }
        allergens.insert(allergen, ingredient);
    }
    allergens.values().cloned().collect::<Vec<_>>().join(",")
}
