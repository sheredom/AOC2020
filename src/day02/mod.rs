#[exec_time]
fn day02_part01(slice: &[(i32, i32, &str, &str)]) {
    let mut total = 0;
    for i in slice.iter() {
        let lo = i.0;
        let hi = i.1;
        let rune = i.2;
        let password = i.3;

        let count = password.matches(rune).count() as i32;

        if count >= lo && count <= hi {
            total += 1;
        }
    }

    println!("Day 02, part 01: Valid passwords {}", total);
}

#[exec_time]
fn day02_part02(slice: &[(i32, i32, &str, &str)]) {
    let mut total = 0;
    for i in slice.iter() {
        // -1 because the indices are using 1-based indexing.
        let lo = i.0 as usize - 1;
        let hi = i.1 as usize - 1;
        let rune = i.2;
        let password = i.3;

        if !((password[lo..lo + 1] == *rune) ^ (password[hi..hi + 1] == *rune)) {
            continue;
        }

        total += 1;
    }

    println!("Day 02, part 02: Valid passwords {}", total);
}

pub fn day02() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let iter: Vec<&str> = string.split_whitespace().collect();

    let mut vec = Vec::new();

    for chunk in iter.chunks(3) {
        let range = chunk.get(0).unwrap();
        let rune = (*chunk.get(1).unwrap()).strip_suffix(":").unwrap();
        let password = *chunk.get(2).unwrap();

        let mut split = range.split('-');
        let lo = split.next().unwrap().parse::<i32>().unwrap();
        let hi = split.next().unwrap().parse::<i32>().unwrap();

        vec.push((lo, hi, rune, password));
    }

    day02_part01(&vec);
    day02_part02(&vec);
}
