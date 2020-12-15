use std::collections::HashMap;

fn calc_for_nth(vec: &[u32], n: u32) -> u32 {
    let mut map = HashMap::new();

    vec.iter().enumerate().for_each(|(index, number)| {
        map.insert(*number, index as u32);
    });

    let mut last = 0;

    for i in (vec.len() as u32)..(n - 1) {
        last = if let Some(value) = map.get_mut(&last) {
            let old = *value;
            *value = i;
            i - old
        } else {
            map.insert(last, i);
            0
        };
    }

    last
}

#[exec_time]
fn day15_part01(vec: &[u32]) {
    let result = calc_for_nth(vec, 2020);

    green_ln!(
        "Day 15, part 01: 2020th number spoken in memory game is {} ",
        result
    );
}

#[exec_time]
fn day15_part02(vec: &[u32]) {
    let result = calc_for_nth(vec, 30000000);

    green_ln!(
        "Day 15, part 02: 30000000th number spoken in memory game is {} ",
        result
    );
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let vec: Vec<_> = string
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    day15_part01(&vec);
    day15_part02(&vec);
}
