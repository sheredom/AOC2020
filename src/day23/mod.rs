#[exec_time]
fn day21_part01(original: Vec<u8>, start: u8) {
    let mut cups = original;
    let cups_len = cups.len();

    let max = (cups_len - 1) as u8;

    let moves = 100;

    let mut current = start;

    for _ in 0..moves {
        let picked_up_0 = cups[current as usize];
        let picked_up_1 = cups[picked_up_0 as usize];
        let picked_up_2 = cups[picked_up_1 as usize];

        let mut next = current - 1;

        loop {
            if next == 0 {
                // Wrap around to the biggest!
                next = max;
                continue;
            } else if next != picked_up_0 && next != picked_up_1 && next != picked_up_2 {
                break;
            }

            next -= 1;
        }

        cups[current as usize] = cups[picked_up_2 as usize];
        cups[picked_up_2 as usize] = cups[next as usize];
        cups[next as usize] = picked_up_0;

        current = cups[current as usize];
    }

    let mut string = String::new();

    let mut next = cups[1];

    while next != 1 {
        string += &next.to_string();
        next = cups[next as usize];
    }

    red_ln!("Day 23, part 01: Cup labels after 1 - {}", string);
}

#[exec_time]
fn day21_part02(original: Vec<u32>, start: u32) {
    let mut cups = original;
    let cups_len = cups.len();

    let max = (cups_len - 1) as u32;

    let moves = 10000000;

    let mut current = start;

    for _ in 0..moves {
        let picked_up_0 = cups[current as usize];
        let picked_up_1 = cups[picked_up_0 as usize];
        let picked_up_2 = cups[picked_up_1 as usize];

        let mut next = current - 1;

        loop {
            if next == 0 {
                // Wrap around to the biggest!
                next = max;
                continue;
            } else if next != picked_up_0 && next != picked_up_1 && next != picked_up_2 {
                break;
            }

            next -= 1;
        }

        cups[current as usize] = cups[picked_up_2 as usize];
        cups[picked_up_2 as usize] = cups[next as usize];
        cups[next as usize] = picked_up_0;

        current = cups[current as usize];
    }

    let first = cups[1] as usize;
    let second = cups[first] as usize;

    green_ln!(
        "Day 23, part 02: Cup labels after 1 - {} * {} = {}",
        first,
        second,
        first * second
    );
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let mut vec = Vec::with_capacity(string.len() + 1);
    vec.resize(string.len() + 1, std::u8::MAX);

    let start = string.chars().next().unwrap().to_digit(10).unwrap() as u8;

    let mut last = start;

    for c in string.chars().skip(1) {
        let next = c.to_digit(10).unwrap() as u8;
        vec[last as usize] = next;
        last = next;
    }

    vec[last as usize] = start;

    day21_part01(vec, start);

    let mut vec = Vec::with_capacity(1000000 + 1);
    vec.resize(1000000 + 1, std::u32::MAX);

    let start = string.chars().next().unwrap().to_digit(10).unwrap() as u32;

    let mut last = start;

    for c in string.chars().skip(1) {
        let next = c.to_digit(10).unwrap() as u32;
        vec[last as usize] = next;
        last = next;
    }

    vec[last as usize] = (string.len() + 1) as u32;

    for (i, item) in vec.iter_mut().enumerate().skip(string.len() + 1) {
        *item = (i + 1) as u32;
    }

    vec[1000000] = start;

    day21_part02(vec, start);
}
