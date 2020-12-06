use intbits::Bits;

#[exec_time]
fn day06_part01(string: &str) {
    let mut total = 0;

    let mut bits: i32 = 0;

    for line in string.lines() {
        if line.is_empty() {
            total += bits.count_ones();

            bits = 0;

            continue;
        }

        for c in line.chars() {
            bits.set_bit(c.to_digit(36).unwrap() - 10, true);
        }
    }

    total += bits.count_ones();

    red_ln!("Day 06, part 01: Total yes groups {}", total);
}

#[exec_time]
fn day06_part02(string: &str) {
    let mut total = 0;

    let mut bits: i32 = -1;

    for line in string.lines() {
        if line.is_empty() {
            total += bits.count_ones();

            bits = -1;

            continue;
        }

        bits &= line.chars().fold(0, |mut acc, c| {
            acc.set_bit(c.to_digit(36).unwrap() - 10, true);
            acc
        });
    }

    total += bits.count_ones();

    green_ln!("Day 06, part 02: Total everybody yes groups {}", total);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day06_part01(&string);
    day06_part02(&string);
}
