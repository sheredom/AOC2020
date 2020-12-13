use itertools::Itertools;
use num_integer::lcm;

fn nextafter(timestamp: u64, bus: u64) -> u64 {
    (timestamp / bus) * (bus) + bus
}

fn folder(acc: (u64, u64), element: (&u64, u64)) -> (u64, u64) {
    if element.1 < acc.1 {
        (*element.0, element.1)
    } else {
        acc
    }
}

#[exec_time]
fn day13_part01(timestamp: u64, buses: &[u64]) {
    let (bus, minutes) = buses
        .iter()
        .map(|bus| (bus, nextafter(timestamp, *bus)))
        .fold((0, std::u64::MAX), folder);

    let result = bus * (minutes - timestamp);

    red_ln!(
        "Day 13, part 01: bus {} * minutes waiting {} = {}",
        bus,
        minutes - timestamp,
        result
    );
}

#[exec_time]
fn day13_part02(buses: &[(u64, u64)]) {
    let mut timestamp = 0;
    let mut step = 1;

    for bus in buses {
        while (timestamp + bus.0) % bus.1 != 0 {
            timestamp += step;
        }

        step = lcm(step, bus.1);
    }

    green_ln!(
        "Day 13, part 02: Earliest timestamp that matches mapping {}",
        timestamp
    );
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let (timestamp, buses) = string.lines().next_tuple().unwrap();

    let timestamp = timestamp.parse::<u64>().unwrap();
    let buses_part01: Vec<u64> = buses
        .split(',')
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse::<u64>().unwrap())
        .collect();

    day13_part01(timestamp, &buses_part01);

    let buses_part02: Vec<(_, _)> = buses
        .split(',')
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        .map(|(index, bus)| (index as _, bus.parse::<_>().unwrap()))
        .collect();

    day13_part02(&buses_part02);
}
