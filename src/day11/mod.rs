fn count_occupied_seats_part01(grid: &[(usize, u8)], width: usize, index: usize) -> usize {
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

    indices.iter().filter(|i| grid[**i].1 == b'#').count()
}

#[exec_time]
fn day11_part01(g: &[u8], width: usize) {
    let mut grid: Vec<(usize, u8)> = g.iter().copied().enumerate().collect();
    let mut next = grid.clone();

    loop {
        for (index, c) in grid.iter() {
            next[*index] = (*index, *c);

            match c {
                _ if *c == b'L' => {
                    if count_occupied_seats_part01(&grid, width, *index) == 0 {
                        next[*index] = (*index, b'#');
                    }
                }
                _ if *c == b'#' => {
                    if count_occupied_seats_part01(&grid, width, *index) >= 4 {
                        next[*index] = (*index, b'L');
                    }
                }
                _ => (),
            }
        }

        if next == grid {
            break;
        }

        std::mem::swap(&mut next, &mut grid);
    }

    let result = grid.iter().filter(|(_, c)| *c == b'#').count();

    red_ln!("Day 11, part 01: Stable occupied seats {}", result);
}

fn count_occupied_seats_part02(grid: &[(usize, u8)], w: usize, index: usize) -> usize {
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
            let x = cur % width;
            let y = cur / width;

            if (x == 0) || (y == 0) || (x == (width - 1)) || (cur as usize) > grid.len() {
                break 0;
            }

            match grid[cur as usize].1 {
              b'#' => break 1,
              b'L' => break 0,
              _ => ()
            }

            cur += offset;
        };
    }

    total
}

#[exec_time]
fn day11_part02(g: &[u8], width: usize) {
    let mut grid: Vec<(usize, u8)> = g.iter().copied().enumerate().collect();
    let mut next = grid.clone();

    loop {
        for (index, c) in grid.iter() {
            next[*index] = (*index, *c);

            match c {
                _ if *c == b'L' => {
                    if count_occupied_seats_part02(&grid, width, *index) == 0 {
                        next[*index] = (*index, b'#');
                    }
                }
                _ if *c == b'#' => {
                    if count_occupied_seats_part02(&grid, width, *index) >= 5 {
                        next[*index] = (*index, b'L');
                    }
                }
                _ => (),
            }
        }

        if next == grid {
            break;
        }

        std::mem::swap(&mut next, &mut grid);
    }

    let result = grid.iter().filter(|(_, c)| *c == b'#').count();

    green_ln!("Day 11, part 02: Stable occupied seats {}", result);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let mut width = None;
    let mut grid: String = String::new();
    let mut empty: String = String::new();
    for line in string.lines() {
        if let Some(w) = width {
            assert_eq!(w, line.len() + 2);
        }

        width = Some(line.len() + 2);

        // add an empty row at the start too.
        if grid.is_empty() {
            empty.push('.');
            empty = empty.repeat(width.unwrap());
            grid.push_str(&empty);
        }

        // Add an empty seat around the perimeter.
        grid.push('.');
        grid.push_str(line);
        grid.push('.');
    }

    // Add an empty row at the end also.
    grid.push_str(&empty);

    day11_part01(grid.as_bytes(), width.unwrap());
    day11_part02(grid.as_bytes(), width.unwrap());
}
