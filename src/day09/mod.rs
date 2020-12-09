use std::cmp::{max, min};

#[exec_time]
fn day09_part01(string: &str) -> i64 {
    let mut invalid = 0;
    let mut window = [0; 25];

    for (index, line) in string.lines().enumerate() {
        invalid = line.parse::<i64>().unwrap();

        // If we haven't accumulated enough to populate the window a first time, bail.
        if index < 25 {
            window[index % 25] = invalid;
            continue;
        }

        let mut found = false;

        for (kndex, k) in window.iter().enumerate() {
            for (mndex, m) in window.iter().enumerate() {
                // SKip ourselves.
                if kndex == mndex {
                    continue;
                }

                if (k + m) == invalid {
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        if !found {
            break;
        }

        window[index % 25] = invalid;
    }

    red_ln!(
        "Day 09, part 01: First value that is not sum of previous 25 {}",
        invalid
    );

    invalid
}

#[exec_time]
fn day09_part02(string: &str, invalid: i64) {
    for (index, _) in string.lines().enumerate() {
        let mut lo = std::i64::MAX;
        let mut hi = std::i64::MIN;
        let mut total = 0;

        for line in string.lines().skip(index) {
            let current = line.parse::<i64>().unwrap();

            lo = min(lo, current);
            hi = max(hi, current);
            total += current;

            if total == invalid {
                let result = lo + hi;

                green_ln!("Day 09, part 02: Sum of low/high of sequence {}", result);

                return;
            }
        }
    }
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let invalid = day09_part01(&string);
    day09_part02(&string, invalid);
}
