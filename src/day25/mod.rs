use itertools::Itertools;

fn break_encryption(key: u64) -> u64 {
    let mut result = 1;

    let mut loop_size = 1;

    loop {
        result *= 7;
        result %= 20201227;

        if result == key {
            return loop_size;
        }

        loop_size += 1;
    }
}

fn encrypt(value: u64, loop_size: u64) -> u64 {
    let mut result = 1;
    for _ in 0..loop_size {
        result *= value;
        result %= 20201227;
    }

    result
}

#[exec_time]
fn day25_part01(my_key: u64, door_key: u64) {
    let my_key_loop_size = break_encryption(my_key);
    let door_key_loop_size = break_encryption(door_key);

    println!("{} {}", my_key_loop_size, door_key_loop_size);

    let encrypted_door_key = encrypt(door_key, my_key_loop_size);
    let encrypted_my_key = encrypt(my_key, door_key_loop_size);

    assert_eq!(encrypted_door_key, encrypted_my_key);

    red_ln!("Day 25, part 01: Encryption key {}", encrypted_my_key);
}

#[exec_time]
fn day25_part02() {
    green_ln!("Day 25, part 02: Got all 50 stars!");
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let (my_key, door_key) = string
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .next_tuple()
        .unwrap();

    day25_part01(my_key, door_key);

    day25_part02();
}
