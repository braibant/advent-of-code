use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space1},
    combinator::{map, map_res},
    IResult,
};

// Each food is described by its list of ingredients, and the list of allergens these contain.
type Food = (Vec<String>, Vec<String>);

mod parsing {
    use super::*;

    fn word(input: &str) -> IResult<&str, String> {
        map(nom::character::complete::alpha1, |c: &str| c.to_string())(input)
    }

    pub fn food(input: &str) -> IResult<&str, Food> {
        let ingredients = nom::multi::separated_list1(space1, word);

        let allergens = nom::sequence::delimited(
            tag(" (contains "),
            nom::multi::separated_list1(tag(", "), word),
            tag(")"),
        );

        let mut food = nom::sequence::tuple((ingredients, allergens));

        food(input)
    }
}

fn possible_sources(allergen: &str, foods: &Vec<Food>) -> HashSet<String> {
    let foods_containing_alergen: Vec<_> = foods
        .iter()
        .filter(|(_ingredients, allergens)| allergens.iter().any(|a| a == allergen))
        .collect();

    let mut possible_sources: HashSet<String> = HashSet::new();

    // Let's seed the set with the list of ingredients of the first food.
    let (ingredients, _allergens) = foods_containing_alergen[0];
    possible_sources.extend(ingredients.iter().cloned());

    // Then, let's build the list of ingredients that appear in _all_ the foods containing the allergen.
    for (ingredients, _allergens) in foods_containing_alergen.iter() {
        let ingredients: HashSet<_> = ingredients.iter().cloned().collect();
        possible_sources = possible_sources
            .intersection(&ingredients)
            .cloned()
            .collect()
    }

    return possible_sources;
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let foods: Vec<Food> = contents
        .split("\n")
        .map(|line| {
            let (_rest, food) = parsing::food(line).unwrap();
            food
        })
        .collect();

    let allergens: HashSet<_> = foods
        .iter()
        .flat_map(|(_ingredients, allergens)| allergens)
        .collect();
    // The key observation here is that `each allergen is found in exactly one ingredient`.

    let all_possible_sources: HashSet<_> = allergens
        .iter()
        .flat_map(|allergen| possible_sources(allergen, &foods))
        .collect();

    println!("{:?}", all_possible_sources);

    let part1 = foods
        .iter()
        .flat_map(|(ingredients, _allergens)| ingredients)
        .filter(|ingredient| !all_possible_sources.contains(ingredient.clone()))
        .count();

    println!("{:?}", part1);

    let start = Instant::now();
    let mut candidates: Vec<_> = allergens
        .iter()
        .map(|allergen| (allergen, possible_sources(allergen, &foods)))
        .collect();

    // The list of pairs `allergen, ingredient` that we have discovered.
    let mut allergens = Vec::new();

    while candidates.len() > 0 {
        candidates.sort_by_key(|(_allergen, sources)| sources.len());

        let (allergen, mut sources) = candidates.remove(0);
        assert_eq!(sources.len(), 1);

        let source = sources.drain().next().unwrap();
        allergens.push((allergen, source.clone()));

        for (_allergen, sources) in candidates.iter_mut() {
            sources.remove(&source);
        }
    }

    // Let's sort the allergen mapping alphabetically by allergen, and then get out the list of ingredients they corrspond to.
    allergens.sort();

    println!("Time {:?}", start.elapsed());

    println!(
        "{}",
        allergens
            .iter()
            .map(|(_, ingredient)| ingredient)
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    )
}
