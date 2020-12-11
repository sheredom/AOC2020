fn count_occupied_seats(grid: &[u8], width: usize, index: usize) -> usize {
    let bytes = grid.iter().enumerate();

    bytes
        .filter(|(kndex, c)| match kndex {
            _ if *kndex == (index - width - 1) => **c == '#' as u8,
            _ if *kndex == (index - width + 0) => **c == '#' as u8,
            _ if *kndex == (index - width + 1) => **c == '#' as u8,
            _ if *kndex == (index - 1) => **c == '#' as u8,
            _ if *kndex == (index + 1) => **c == '#' as u8,
            _ if *kndex == (index + width - 1) => **c == '#' as u8,
            _ if *kndex == (index + width + 0) => **c == '#' as u8,
            _ if *kndex == (index + width + 1) => **c == '#' as u8,
            _ => false,
        })
        .count()
}

#[exec_time]
fn day11_part01(g: &[u8], width: usize) {
    let mut grid: Vec<u8> = g.iter().map(|x| *x).collect();
    let mut next = grid.clone();

    loop {
        for (index, c) in grid.iter().enumerate() {
            next[index] = *c;

            match c {
                _ if *c == 'L' as u8 => {
                    if count_occupied_seats(&grid, width, index) == 0 {
                        next[index] = '#' as u8;
                    }
                }
                _ if *c == '#' as u8 => {
                    if count_occupied_seats(&grid, width, index) >= 4 {
                        next[index] = 'L' as u8;
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

    let result = grid.iter().filter(|c| **c == '#' as u8).count();

    red_ln!("Day 11, part 01: Stable occupied seats {}", result);
}

#[exec_time]
fn day11_part02(grid: &str, width: usize) {

    //green_ln!("Day 11, part 02: Distinct arrangements {}", result);
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
        if grid.len() == 0 {
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
    day11_part02(&grid, width.unwrap());
}
