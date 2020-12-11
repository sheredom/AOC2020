fn count_occupied_seats_part01(grid: &[Types], width: usize, index: usize) -> usize {
    let indices = vec![
        index - width - 1,
        index - width,
        index - width + 1,
        index - 1,
        index + 1,
        index + width - 1,
        index + width,
        index + width + 1,
    ];

    indices
        .iter()
        .filter(|i| grid[**i] == Types::Occupied)
        .count()
}

#[exec_time]
fn day11_part01(g: &[Types], width: usize) {
    let mut grid = g.to_vec();
    let mut pending = Vec::new();

    loop {
        for (index, c) in grid.iter().enumerate() {
            match c {
                Types::Empty if count_occupied_seats_part01(&grid, width, index) == 0 => {
                    pending.push((index, Types::Occupied));
                }
                Types::Occupied if count_occupied_seats_part01(&grid, width, index) >= 4 => {
                    pending.push((index, Types::Empty));
                }
                _ => (),
            }
        }

        if pending.is_empty() {
            break;
        }

        for p in &pending {
            grid[p.0] = p.1;
        }

        pending.clear();
    }

    let result = grid.iter().filter(|t| **t == Types::Occupied).count();

    red_ln!("Day 11, part 01: Stable occupied seats {}", result);
}

fn count_occupied_seats_part02(grid: &[Types], w: usize, index: usize) -> usize {
    let width = w as isize;

    let offsets = vec![
        -width - 1,
        -width,
        -width + 1,
        -1,
        1,
        width - 1,
        width,
        width + 1,
    ];

    let mut total = 0;

    for offset in offsets {
        let mut cur = (index as isize) + offset;
        total += loop {
            match grid[cur as usize] {
                Types::Occupied => break 1,
                Types::Empty => break 0,
                Types::Boundary => break 0,
                _ => (),
            }

            cur += offset;
        };
    }

    total
}

#[exec_time]
fn day11_part02(g: &[Types], width: usize) {
    let mut grid = g.to_vec();
    let mut pending = Vec::new();

    loop {
        for (index, c) in grid.iter().enumerate() {
            match c {
                Types::Empty if count_occupied_seats_part02(&grid, width, index) == 0 => {
                    pending.push((index, Types::Occupied));
                }
                Types::Occupied if count_occupied_seats_part02(&grid, width, index) >= 5 => {
                    pending.push((index, Types::Empty));
                }
                _ => (),
            }
        }

        if pending.is_empty() {
            break;
        }

        for p in &pending {
            grid[p.0] = p.1;
        }

        pending.clear();
    }

    let result = grid.iter().filter(|t| **t == Types::Occupied).count();

    green_ln!("Day 11, part 02: Stable occupied seats {}", result);
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Types {
    Nothing = 0,
    Boundary = 1,
    Empty = 2,
    Occupied = 3,
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let mut width = None;
    let mut vec = Vec::new();

    for line in string.lines() {
        if let Some(w) = width {
            assert_eq!(w, line.len() + 2);
        } else {
            width = Some(line.len() + 2);

            for _ in 0..width.unwrap() {
                vec.push(Types::Boundary);
            }
        }

        // Start with a boundary.
        vec.push(Types::Boundary);

        line.chars().for_each(|c| match c {
            '.' => vec.push(Types::Nothing),
            'L' => vec.push(Types::Empty),
            '#' => vec.push(Types::Occupied),
            _ => panic!("Unknown char types"),
        });

        // End with a boundary.
        vec.push(Types::Boundary);
    }

    // Add an empty row at the end also.
    for _ in 0..width.unwrap() {
        vec.push(Types::Boundary);
    }

    day11_part01(&vec, width.unwrap());
    day11_part02(&vec, width.unwrap());
}
