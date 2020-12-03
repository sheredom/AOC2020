use intbits::Bits;

fn slope_calculate(slice: &[i32], num_bits: i32, x_slope: i32, y_slope: i32) -> i32 {
    let mut total = 0;

    let mut x = 0;

    for y in slice.iter().step_by(y_slope as usize) {
        if y.bit(x % num_bits) {
            total += 1;
        }

        x += x_slope;
    }

    total
}

#[exec_time]
fn day03_part01(slice: &[i32], num_bits: i32) {
    let total = slope_calculate(slice, num_bits, 3, 1);

    println!("Day 03, part 01: Number of trees {}", total);
}

#[exec_time]
fn day03_part02(slice: &[i32], num_bits: i32) {
    let mut total = 1;
    total *= slope_calculate(slice, num_bits, 1, 1);
    total *= slope_calculate(slice, num_bits, 3, 1);
    total *= slope_calculate(slice, num_bits, 5, 1);
    total *= slope_calculate(slice, num_bits, 7, 1);
    total *= slope_calculate(slice, num_bits, 1, 2);

    println!("Day 03, part 02: Number of trees {}", total);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let num_bits = string.lines().map(|s| s.len()).max().unwrap();
    assert!(num_bits < 32);

    let mut vec = Vec::new();

    for line in string.lines() {
        assert_eq!(line.len(), num_bits);

        let mut i: i32 = 0;

        for (index, c) in line.chars().enumerate() {
            match c {
                '.' => i.set_bit(index, false),
                '#' => i.set_bit(index, true),
                _ => panic!("Unknown character in input!"),
            }
        }

        vec.push(i);
    }

    day03_part01(&vec, num_bits as i32);
    day03_part02(&vec, num_bits as i32);
}
