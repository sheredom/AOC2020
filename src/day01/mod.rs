#[exec_time]
fn day01_part01(slice: &[i32]) {
    for (index, i) in slice.iter().enumerate() {
        // Skip i's that are already to big!
        if *i > 2020 {
            continue;
        }

        for (kndex, k) in slice.iter().enumerate() {
            // Skip indices that we will already have compared.
            if kndex <= index {
                continue;
            }

            if (i + k) == 2020 {
                println!(
                    "Day 01, part 01: At indices ({}, {}) with values ({}, {}) found the answer: {}",
                    index,
                    kndex,
                    i,
                    k,
                    i * k
                );
                return;
            }
        }
    }
}

#[exec_time]
fn day01_part02(slice: &[i32]) {
    for (index, i) in slice.iter().enumerate() {
        // Skip i's that are already to big!
        if *i > 2020 {
            continue;
        }

        // Skip indices that we will already have compared.
        for (kndex, k) in slice.iter().enumerate() {
            if kndex <= index {
                continue;
            }

            // Skip i + k's that are already to big!
            if (i + k) > 2020 {
                continue;
            }

            for (mndex, m) in slice.iter().enumerate() {
                // Skip indices that we will already have compared.
                if mndex <= kndex {
                    continue;
                }

                if (i + k + m) == 2020 {
                    println!(
                    "Day 01, part 01: At indices ({}, {}, {}) with values ({}, {}, {}) found the answer: {}",
                    index,
                    kndex,
                    mndex,
                    i,
                    k,
                    m,
                    i * k * m
                );
                    return;
                }
            }
        }
    }
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let vec: Vec<i32> = string
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    day01_part01(&vec);
    day01_part02(&vec);
}
