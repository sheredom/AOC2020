use std::collections::{HashMap, HashSet};

fn is_valid_set(set: &mut HashSet<&str>) -> bool {
    // Get rid of the optional cid.
    set.remove("cid");

    // There are 7 valid IDs for each passport, check they all exist.
    set.len() == 7
}

#[exec_time]
fn day04_part01(string: &str) {
    let mut valid = 0;

    let mut set = HashSet::new();

    for line in string.lines() {
        // IF we have an empty line we reset our counts.
        if line.len() == 0 {
            if is_valid_set(&mut set) {
                valid += 1;
            }

            set.clear();

            continue;
        }

        for split in line.split_whitespace() {
            set.insert(split.split_at(3).0);
        }
    }

    // One last check for the end of the file.
    set.remove("cid");

    if is_valid_set(&mut set) {
        valid += 1;
    }

    println!("Day 04, part 01: Valid passports {}", valid);
}

fn is_valid_map(map: &mut HashMap<&str, &str>) -> bool {
    // Get rid of the optional cid.
    map.remove("cid");

    // There are 7 valid IDs for each passport, check they all exist.
    if map.len() != 7 {
        return false;
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = map.get("byr").unwrap();

    if byr.len() != 4 {
        return false;
    }

    let byr_as_int = byr.parse::<i32>().unwrap_or(-1);

    if byr_as_int < 1920 || byr_as_int > 2002 {
        return false;
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = map.get("iyr").unwrap();

    if iyr.len() != 4 {
        return false;
    }

    let iyr_as_int = iyr.parse::<i32>().unwrap_or(-1);

    if iyr_as_int < 2010 || iyr_as_int > 2020 {
        return false;
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = map.get("eyr").unwrap();

    if eyr.len() != 4 {
        return false;
    }

    let eyr_as_int = eyr.parse::<i32>().unwrap_or(-1);

    if eyr_as_int < 2020 || eyr_as_int > 2030 {
        return false;
    }

    // hgt (Height) - a number followed by either cm or in:
    let hgt = map.get("hgt").unwrap();

    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    if let Some(cm) = hgt.strip_suffix("cm") {
        let cm_as_int = cm.parse::<i32>().unwrap_or(-1);
        if cm_as_int < 150 || cm_as_int > 193 {
            return false;
        }
    } else if let Some(r#in) = hgt.strip_suffix("in") {
        let in_as_int = r#in.parse::<i32>().unwrap_or(-1);
        if in_as_int < 59 || in_as_int > 76 {
            return false;
        }
    } else {
        return false;
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl = map.get("hcl").unwrap();

    if let Some(colour) = hcl.strip_prefix('#') {
        if colour.len() != 6 {
            return false;
        }

        if colour.contains(|c: char| !c.is_ascii_hexdigit()) {
            return false;
        }
    } else {
        return false;
    }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    match *map.get("ecl").unwrap() {
        "amb" => (),
        "blu" => (),
        "brn" => (),
        "gry" => (),
        "grn" => (),
        "hzl" => (),
        "oth" => (),
        _ => return false,
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid = map.get("pid").unwrap();

    if pid.len() != 9 {
        return false;
    }

    if pid.contains(|c: char| !c.is_numeric()) {
        return false;
    }

    true
}

#[exec_time]
fn day04_part02(string: &str) {
    let mut valid = 0;

    let mut map = HashMap::new();

    for line in string.lines() {
        // IF we have an empty line we reset our counts.
        if line.len() == 0 {
            if is_valid_map(&mut map) {
                valid += 1;
            }

            map.clear();

            continue;
        }

        for split in line.split_whitespace() {
            let tuple = split.split_at(3);
            map.insert(tuple.0, tuple.1.split_at(1).1);
        }
    }

    // One last check for the end of the file.
    if is_valid_map(&mut map) {
        valid += 1;
    }

    println!("Day 04, part 02: Valid passports {}", valid);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day04_part01(&string);
    day04_part02(&string);
}
