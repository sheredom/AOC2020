use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

#[exec_time]
fn day16_part01(ranges: &[(String, std::ops::Range<i32>, std::ops::Range<i32>)], tickets: &str) {
    let mut total = 0;

    for ticket in tickets.lines() {
        for number in ticket.split(',') {
            let i = number.parse::<i32>().unwrap();

            if !ranges
                .iter()
                .any(|(_, r0, r1)| r0.contains(&i) || r1.contains(&i))
            {
                total += i;
            }
        }
    }

    red_ln!("Day 16, part 01: Scanned ticket error rate {} ", total);
}

#[exec_time]
fn day16_part02(
    ranges: &[(String, std::ops::Range<i32>, std::ops::Range<i32>)],
    tickets: &str,
    my_ticket: &str,
) {
    let mut valid_tickets = Vec::new();

    for ticket in tickets.lines() {
        let mut valid = true;

        for number in ticket.split(',') {
            let i = number.parse::<i32>().unwrap();

            if !ranges
                .iter()
                .any(|(_, r0, r1)| r0.contains(&i) || r1.contains(&i))
            {
                valid = false;
                break;
            }
        }

        if valid {
            valid_tickets.push(ticket);
        }
    }

    let mut valid_per_ticket: Vec<HashSet<&str>> = Vec::new();

    for _ in 0..my_ticket.split(',').count() {
        let mut set = HashSet::new();

        ranges.iter().for_each(|(name, _, _)| {
            set.insert(name.as_str());
        });

        valid_per_ticket.push(set);
    }

    for ticket in valid_tickets {
        for (index, number) in ticket.split(',').enumerate() {
            let i = number.parse::<i32>().unwrap();

            ranges.iter().for_each(|(name, r0, r1)| {
                if !r0.contains(&i) && !r1.contains(&i) {
                    valid_per_ticket[index].remove(name.as_str());
                }
            });
        }
    }

    // Now we should have at least one number in the ticket that we can map to
    // a single field, and thus we can remove it from all other fields!
    loop {
        let mut to_process = Vec::new();
        for valid in &valid_per_ticket {
            if valid.len() == 1 {
                to_process.push(valid.iter().next().unwrap().to_string());
            }
        }

        if to_process.len() == valid_per_ticket.len() {
            break;
        }

        for process in to_process {
            for valid in &mut valid_per_ticket {
                if valid.len() != 1 {
                    valid.remove(process.as_str());
                }
            }
        }
    }

    let mut total = 1;

    for (name, number) in valid_per_ticket
        .iter()
        .map(|s| s.iter().next().unwrap())
        .zip(my_ticket.split(','))
    {
        if name.starts_with("departure") {
            total *= number.parse::<i64>().unwrap();
        }
    }

    green_ln!("Day 16, part 02: Departure multiplies {} ", total);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let (conditions, string) = string.splitn(2, "your ticket:").next_tuple().unwrap();
    let (my_ticket, tickets) = string.splitn(2, "nearby tickets:").next_tuple().unwrap();

    let my_ticket = my_ticket.trim();
    let tickets = tickets.trim();

    let re = Regex::new(r"(.*): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();

    let ranges: Vec<_> = re
        .captures_iter(conditions)
        .map(|cap| {
            (
                cap[1].to_string(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
                cap[5].parse::<i32>().unwrap(),
            )
        })
        .map(|(name, x_lo, x_hi, y_lo, y_hi)| (name, (x_lo..(x_hi + 1)), (y_lo..(y_hi + 1))))
        .collect();

    day16_part01(&ranges, &tickets);
    day16_part02(&ranges, &tickets, &my_ticket);
}
