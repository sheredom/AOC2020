use intbits::Bits;
use itertools::Itertools;
use std::collections::HashMap;

#[exec_time]
fn day14_part01(string: &str) {
    let mut or_mask = 0;
    let mut and_mask = 0;

    let mut map: HashMap<u64, u64> = HashMap::new();

    for line in string.lines() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            // Wipe the existing masks.
            or_mask = 0;
            and_mask = 0;

            for c in mask.chars() {
                or_mask <<= 1;
                and_mask <<= 1;

                match c {
                    '1' => or_mask |= 0b1,
                    '0' => (),
                    'X' => and_mask |= 0b1,
                    _ => panic!("Unknown char {}", c),
                }
            }
        } else if let Some(mem) = line.strip_prefix("mem[") {
            let (address_str, remainder) = mem.splitn(2, ']').next_tuple().unwrap();
            let address = address_str.parse::<u64>().unwrap();
            let value = remainder
                .strip_prefix(" = ")
                .unwrap()
                .parse::<u64>()
                .unwrap();

            map.insert(address, (value & and_mask) | or_mask);
        }
    }

    let result = map.iter().fold(0, |acc, (_, value)| acc + value);

    red_ln!(
        "Day 14, part 01: sum of all values in memory is {} ",
        result
    );
}

fn scatter_bits(mut bits: u64, mut mask: u64) -> u64 {
    let mut result = 0;

    while bits != 0 && mask != 0 {
        let lowest = mask.trailing_zeros();

        mask.set_bit(lowest, false);

        result.set_bit(lowest, bits.bit(0));

        bits >>= 1;
    }

    result
}

#[exec_time]
fn day14_part02(string: &str) {
    let mut or_mask = 0;
    let mut float_mask: u64 = 0;

    let mut map: HashMap<u64, u64> = HashMap::new();

    for line in string.lines() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            // Wipe the existing masks.
            or_mask = 0;
            float_mask = 0;

            for c in mask.chars() {
                or_mask <<= 1;
                float_mask <<= 1;

                match c {
                    '1' => or_mask |= 0b1,
                    '0' => (),
                    'X' => float_mask |= 0b1,
                    _ => panic!("Unknown char {}", c),
                }
            }
        } else if let Some(mem) = line.strip_prefix("mem[") {
            let (address_str, remainder) = mem.splitn(2, ']').next_tuple().unwrap();
            let address = address_str.parse::<u64>().unwrap();
            let value = remainder
                .strip_prefix(" = ")
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let partial_address = (address | or_mask) & !float_mask;

            for float_combo in 0..(1 << float_mask.count_ones()) {
                let used_address = partial_address | scatter_bits(float_combo, float_mask);

                map.insert(used_address, value);
            }
        }
    }

    let result = map.iter().fold(0, |acc, (_, value)| acc + value);

    green_ln!("Day 14, part 02: Sum of all values in memory {}", result);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day14_part01(&string);
    day14_part02(&string);
}
