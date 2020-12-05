use bit_set::BitSet;

fn calculate_seat(bsp: &str) -> (i32, i32) {
    assert_eq!(bsp.len(), 10);

    let (row_str, col_str) = bsp.split_at(7);

    let mut row = 0;

    for (index, c) in row_str.chars().enumerate() {
        let additive = 64 >> index;

        match c {
            'B' => row += additive,
            'F' => (),
            _ => panic!("Unknown character {}", c),
        }
    }

    let mut col = 0;

    for (index, c) in col_str.chars().enumerate() {
        let additive = 4 >> index;

        match c {
            'R' => col += additive,
            'L' => (),
            _ => panic!("Unknown character {}", c),
        }
    }

    (row, col)
}

#[exec_time]
fn day05_part01(string: &str) {
    let mut highest_seat_id = 0;

    for line in string.lines() {
        let (row, col) = calculate_seat(line);

        let seat_id = row * 8 + col;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    println!("Day 05, part 01: Highest seat id {}", highest_seat_id);
}

#[exec_time]
fn day05_part02(string: &str) {
    let mut seats = BitSet::with_capacity(1000);

    for line in string.lines() {
        let (row, col) = calculate_seat(line);

        let seat_id = row * 8 + col;

        seats.insert(seat_id as usize);
    }

    let mut my_seat_id = 0;

    for i in 1..999 {
        if !seats.contains(i) && seats.contains(i - 1) && seats.contains(i + 1) {
            my_seat_id = i;
            break;
        }
    }

    println!("Day 05, part 02: My seat id {}", my_seat_id);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day05_part01(&string);
    day05_part02(&string);
}
