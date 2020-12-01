fn day01_part01(str: &str) {
    let iter = str.split_whitespace();

    for (index, i_string) in iter.clone().enumerate() {
        let i = i_string.parse::<i32>().unwrap();

        // Skip i's that are already to big!
        if i > 2020 {
            continue;
        }

        for (kndex, k_string) in iter.clone().enumerate() {
            // Skip indices that we will already have compared.
            if kndex <= index {
                continue;
            }

            let k = k_string.parse::<i32>().unwrap();

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

fn day01_part02(str: &str) {
    let iter = str.split_whitespace();

    for (index, i_string) in iter.clone().enumerate() {
        let i = i_string.parse::<i32>().unwrap();

        // Skip i's that are already to big!
        if i > 2020 {
            continue;
        }

        // Skip indices that we will already have compared.
        for (kndex, k_string) in iter.clone().enumerate() {
            if kndex <= index {
                continue;
            }

            let k = k_string.parse::<i32>().unwrap();

            // Skip i + k's that are already to big!
            if (i + k) > 2020 {
                continue;
            }

            for (mndex, m_string) in iter.clone().enumerate() {
                // Skip indices that we will already have compared.
                if mndex <= kndex {
                    continue;
                }

                let m = m_string.parse::<i32>().unwrap();

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

pub fn day01() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day01_part01(&string);
    day01_part02(&string);
}
