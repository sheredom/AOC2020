use itertools::Itertools;
use std::collections::HashMap;

type Payload<'a> = (HashMap<&'a str, usize>, Vec<Vec<(usize, usize)>>);

fn parse_data(string: &str) -> Payload {
    let mut inc_id = 0;
    let mut map = HashMap::new();
    let mut vec = Vec::new();

    for line in string.lines() {
        let (mut key, mut remainder) = line.splitn(2, "bags").next_tuple().unwrap();

        key = key.trim();
        remainder = remainder
            .trim_start()
            .trim_start_matches("contain")
            .trim_start();

        // Insert a unique bag id if we hadn't found it already.
        if !map.contains_key(key) {
            map.insert(key, inc_id);
            inc_id += 1;
            vec.resize(inc_id, Vec::new());
        }

        let id = *map.get(key).unwrap();

        while remainder.starts_with(char::is_numeric) {
            let num_bags = remainder
                .split_at(remainder.find(|c: char| !c.is_numeric()).unwrap())
                .0
                .parse::<usize>()
                .unwrap();

            remainder = remainder.trim_start_matches(char::is_numeric).trim_start();

            let (mut sub_key, sub_remainder) = remainder.splitn(2, "bag").next_tuple().unwrap();

            sub_key = sub_key.trim();
            remainder = sub_remainder
                .trim_start_matches("bag")
                .trim_start_matches('s')
                .trim_start_matches(',')
                .trim_start();

            // Insert a unique bag id if we hadn't found it already.
            if !map.contains_key(sub_key) {
                map.insert(sub_key, inc_id);
                inc_id += 1;
                vec.resize(inc_id, Vec::new());
            }

            let sub_id = *map.get(sub_key).unwrap();

            vec[id].push((num_bags, sub_id));
        }
    }

    (map, vec)
}

fn contains_any_gold(shiny_gold_id: usize, id: usize, vec: &[Vec<(usize, usize)>]) -> bool {
    if id == shiny_gold_id {
        true
    } else {
        vec[id]
            .iter()
            .any(|sub_id| contains_any_gold(shiny_gold_id, sub_id.1, vec))
    }
}

#[exec_time]
fn day07_part01(string: &str) {
    let (map, vec) = parse_data(string);

    let shiny_gold_id = *map.get("shiny gold").unwrap();

    // -1 because we want to exclude our shiny gold bag from the search!
    let total = map.values().fold(-1, |acc, id| {
        acc + contains_any_gold(shiny_gold_id, *id, &vec) as i32
    });

    red_ln!(
        "Day 07, part 01: Total bags with at least one shiny bag within {}",
        total
    );
}

fn count_total_bags(id: usize, vec: &[Vec<(usize, usize)>]) -> usize {
    vec[id].iter().fold(1, |acc, sub_id| {
        acc + sub_id.0 * count_total_bags(sub_id.1, &vec)
    })
}

#[exec_time]
fn day07_part02(string: &str) {
    let (map, vec) = parse_data(string);

    let shiny_gold_id = *map.get("shiny gold").unwrap();

    // -1 because the gold bag itself is excluded!
    let total = count_total_bags(shiny_gold_id, &vec) - 1;

    green_ln!("Day 07, part 02: Total bags in a shiny gold bag {}", total);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day07_part01(&string);
    day07_part02(&string);
}
