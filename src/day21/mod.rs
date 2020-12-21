use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[exec_time]
fn day21_part01<'a>(string: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<_, Vec<&str>> = HashMap::new();

    for line in string.lines() {
        let (ingredients, allergies) = line.splitn(2, "(contains ").next_tuple().unwrap();

        let ingredients: Vec<_> = ingredients.split_whitespace().collect();

        let allergies = allergies.strip_suffix(')').unwrap();

        let allergies: Vec<_> = allergies.split(", ").collect();

        for allergy in allergies {
            if let Some(values) = map.get_mut(allergy) {
                // If we have an item, we need to match the current known ingredients that could
                // contain the allergy against the new list, removing any that aren't in both lists.

                let mut to_keep = Vec::new();

                for value in values.iter() {
                    if ingredients.iter().position(|x| *x == *value).is_some() {
                        to_keep.push(*value);
                    }
                }

                *values = to_keep;
            } else {
                // Otherwise the map doesn't already contain the allergy, just insert the current list.
                map.insert(allergy, ingredients.clone());
            }
        }
    }

    let mut total = 0;

    for line in string.lines() {
        let (ingredients, _) = line.splitn(2, "(contains ").next_tuple().unwrap();

        let ingredients: Vec<_> = ingredients.split_whitespace().collect();

        for ingredient in ingredients {
            let mut found = false;

            for (_, values) in map.iter() {
                if values.iter().find(|value| **value == ingredient).is_some() {
                    found = true;
                }
            }

            if !found {
                total += 1;
            }
        }
    }

    red_ln!(
        "Day 21, part 01: Ingredients with no allergens appear {}",
        total
    );

    map
}

#[exec_time]
fn day21_part02(original: HashMap<&str, Vec<&str>>) {
    // We need to de-dup elements in the map until we have a single allergen
    // mapping to a single ingredient. We do this by finding the element with
    // one ingredient, and removing that from all other ingredient lists, until
    // we end up with a 1:1 mapping.
    let mut map = original;

    let mut to_process: HashSet<_> = map.keys().map(|key| *key).collect();

    while !to_process.is_empty() {
        if let Some(next) = to_process.iter().filter(|key| map.get(*key).unwrap().len() == 1).next() {
            let next = *next;
            to_process.remove(next);
            let ingredient = map.get(next).unwrap()[0];

            for (key, values) in map.iter_mut() {
                // Skip ourselves.
                if *key == next {
                    continue;
                }

                if let Some(index) = values.iter().position(|x| *x == ingredient) {
                    values.remove(index);
                }
            }
        } else {
            break;
        }
    }

    let mut sorted_keys: Vec<_> = map.keys().map(|key| *key).collect();
    sorted_keys.sort();

    let mut dangerous_ingredients = String::new();
    for key in sorted_keys {
        dangerous_ingredients += &map.get(key).unwrap()[0].to_string();
        dangerous_ingredients += ",";
    }

    let dangerous_ingredients = dangerous_ingredients.strip_suffix(',').unwrap();

    green_ln!("Day 21, part 02: Dangerous ingredients '{}'", dangerous_ingredients);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let map = day21_part01(&string);
    day21_part02(map);
}
