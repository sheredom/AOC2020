use std::collections::HashMap;

fn differer(mut acc: (i32, i32, i32), x: &i32) -> (i32, i32, i32) {
    match *x - acc.0 {
        1 => acc.1 += 1,
        3 => acc.2 += 1,
        _ => panic!("Unexpected difference {}", *x - acc.0),
    };

    acc.0 = *x;

    acc
}

#[exec_time]
fn day10_part01(vec: &Vec<i32>) {
    let (_, aon, tri) = vec.iter().fold((0, 0, 1), differer);

    let result = aon * tri;

    red_ln!(
        "Day 10, part 01: 1-jolt by 3-jolt differences {} * {} = {}",
        aon,
        tri,
        result
    );
}

// Work out how many combinations of < 3 steps are in the slice.
fn combos(i: i32, slice: &[i32], cached: &mut HashMap<i32, i64>) -> i64 {
    if slice.is_empty() {
        return 1;
    }

    if let Some(cache) = cached.get(&i) {
        return *cache;
    }

    let mut total = 0;

    for (index, x) in slice.iter().enumerate() {
        if (x - i) <= 3 {
            let combo = combos(slice[index], &slice[(index + 1)..], cached);

            cached.insert(slice[index], combo);

            total += combo;
        } else {
            break;
        }
    }

    total
}

#[exec_time]
fn day10_part02(vec: &Vec<i32>) {
    let mut cached = HashMap::new();
    let result = combos(0, &vec, &mut cached);

    green_ln!("Day 10, part 02: Distinct arrangements {}", result);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let mut vec: Vec<i32> = string.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    vec.sort();

    day10_part01(&vec);
    day10_part02(&vec);
}
