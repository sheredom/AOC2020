use bit_reverse::BitwiseReverse;
use bit_set::BitSet;
use intbits::Bits;
use std::collections::HashSet;

fn get_tile_boundaries(data: i128, tile_width: usize) -> (u16, u16, u16, u16) {
    let north = get_row(data, 0, tile_width);
    let east = get_col(data, tile_width - 1, tile_width);
    let south = get_row(data, tile_width - 1, tile_width);
    let west = get_col(data, 0, tile_width);

    (north, east, south, west)
}

fn get_col(tile: i128, col: usize, tile_width: usize) -> u16 {
    let mut ret = 0;

    for i in 0..tile_width {
        ret.set_bit(i, tile.bit(i * tile_width + col));
    }

    ret
}

fn get_row(tile: i128, row: usize, tile_width: usize) -> u16 {
    let mut ret = 0;

    for i in 0..tile_width {
        ret.set_bit(i, tile.bit(row * tile_width + i));
    }

    ret
}

fn set_row(tile: &mut i128, row: usize, tile_width: usize, data: u16) {
    for i in 0..tile_width {
        tile.set_bit(row * tile_width + i, data.bit(i));
    }
}

fn set_col(tile: &mut i128, col: usize, tile_width: usize, data: u16) {
    for i in 0..tile_width {
        tile.set_bit(i * tile_width + col, data.bit(i));
    }
}

fn flip_row(x: u16, tile_width: usize) -> u16 {
    x.swap_bits() >> (16 - tile_width)
}

fn flip_tile_boundaries(
    (n, e, s, w): (u16, u16, u16, u16),
    tile_width: usize,
) -> (u16, u16, u16, u16) {
    (
        flip_row(n, tile_width),
        flip_row(e, tile_width),
        flip_row(s, tile_width),
        flip_row(w, tile_width),
    )
}

fn flip_horizontal(tile: i128, tile_width: usize) -> i128 {
    let mut ret = 0;

    for i in 0..tile_width {
        set_row(
            &mut ret,
            i,
            tile_width,
            flip_row(get_row(tile, i, tile_width), tile_width),
        );
    }

    ret
}

fn rotate_clockwise(tile: i128, tile_width: usize) -> i128 {
    let mut ret = 0;

    for i in 0..tile_width {
        set_col(
            &mut ret,
            tile_width - 1 - i,
            tile_width,
            get_row(tile, i, tile_width),
        );
    }

    ret
}

fn make_match(x: i128, y: i128, tile_width: usize, x_is_north_or_west: bool) -> Option<i128> {
    let (_, xe, xs, _) = get_tile_boundaries(x, tile_width);

    let mut vec = Vec::new();

    vec.push(y);

    let mut tile = y;

    for _ in 0..3 {
        tile = rotate_clockwise(tile, tile_width);
        vec.push(tile);
    }

    tile = flip_horizontal(y, tile_width);
    vec.push(tile);

    for _ in 0..3 {
        tile = rotate_clockwise(tile, tile_width);
        vec.push(tile);
    }

    tile = rotate_clockwise(tile, tile_width);
    tile = flip_horizontal(tile, tile_width);
    vec.push(tile);

    for _ in 0..3 {
        tile = rotate_clockwise(tile, tile_width);
        vec.push(tile);
    }

    if x_is_north_or_west {
        for elem in vec {
            let (yn, _, _, _) = get_tile_boundaries(elem, tile_width);

            if xs == yn {
                return Some(elem);
            }
        }
    } else {
        for elem in vec {
            let (_, _, _, yw) = get_tile_boundaries(elem, tile_width);

            if xe == yw {
                return Some(elem);
            }
        }
    }

    None
}

fn rotate_big_grid(grid: BitSet, width: usize) -> BitSet {
    let mut new_grid = BitSet::new();

    for y in 0..width {
        for x in 0..width {
            let original_index = y * width + x;
            let new_index = x * width + (width - y - 1);

            if grid.contains(original_index) {
                new_grid.insert(new_index);
            }
        }
    }

    new_grid
}

fn flip_big_grid(grid: BitSet, width: usize) -> BitSet {
    let mut new_grid = BitSet::new();

    for y in 0..width {
        for x in 0..width {
            let original_index = y * width + x;
            let new_index = (width - x - 1) * width + (width - y - 1);

            if grid.contains(original_index) {
                new_grid.insert(new_index);
            }
        }
    }

    new_grid
}

#[exec_time]
fn day20_part01(tiles: &[(i16, i128)], tile_width: usize) -> Vec<Vec<i16>> {
    let mut matches = Vec::new();
    matches.resize(tiles.len(), Vec::new());

    for (index, tile) in tiles.iter().enumerate() {
        let (n, e, s, w) = get_tile_boundaries(tile.1, tile_width);
        let (nf, ef, sf, wf) = flip_tile_boundaries((n, e, s, w), tile_width);

        for other_tile in tiles {
            if tile.0 == other_tile.0 {
                continue;
            }

            let (on, oe, os, ow) = get_tile_boundaries(other_tile.1, tile_width);
            let (onf, oef, osf, owf) = flip_tile_boundaries((on, oe, os, ow), tile_width);

            let mut found = false;

            for i in &[n, e, s, w, nf, ef, sf, wf] {
                for k in &[on, oe, os, ow, onf, oef, osf, owf] {
                    if i == k {
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }

            if found {
                matches[index].push(other_tile.0);
            }
        }
    }

    let mut total = 1;

    for (index, per_tile_matches) in matches.iter().enumerate() {
        // Corners will have exactly 2 matches.
        if per_tile_matches.len() == 2 {
            total *= tiles[index].0 as usize;
        }
    }

    red_ln!("Day 20, part 01: Corner multiply {}", total);

    matches
}

#[exec_time]
fn day20_part02(tiles: &[(i16, i128)], tile_width: usize, original: Vec<Vec<i16>>) {
    let mut tile_ids_to_process = Vec::new();
    let mut matches: Vec<_> = original
        .iter()
        .enumerate()
        .map(|(index, list)| (tiles[index], list))
        .collect();

    for tile in tiles {
        tile_ids_to_process.push(tile.0);
    }

    let mut tile_grid = Vec::new();
    tile_grid.resize(tiles.len(), None);
    let tile_grid_width = (tiles.len() as f64).sqrt() as usize;

    let first = matches
        .iter()
        .find(|(_, per_tile_matches)| per_tile_matches.len() == 2)
        .unwrap()
        .0;

    matches.remove(matches.iter().position(|e| e.0 == first).unwrap());

    tile_grid[0] = Some(first);

    for y in 0..tile_grid_width {
        for x in 0..tile_grid_width {
            let tile_grid_element = &tile_grid[y * tile_grid_width + x];

            // Skip anything we've already worked out.
            if tile_grid_element.is_some() {
                continue;
            }
            let prev_y = match y {
                0 => None,
                _ => Some(tile_grid[(y - 1) * tile_grid_width + x].unwrap()),
            };

            let prev_x = match x {
                0 => None,
                _ => Some(tile_grid[y * tile_grid_width + x - 1].unwrap()),
            };

            let mut set = HashSet::new();

            if let Some(prev_y) = prev_y {
                set.insert(prev_y.0);
            }

            if let Some(prev_x) = prev_x {
                set.insert(prev_x.0);
            }

            let mut next = None;
            let mut len = 0;
            let mut max = 0;

            for m in matches.iter().map(|(id, per_tile_matches)| {
                let mut total = 0;

                for per_tile_match in per_tile_matches.iter() {
                    if set.contains(per_tile_match) {
                        total += 1;
                    }
                }

                (id, total, per_tile_matches.len())
            }) {
                if (m.1 > max) || ((m.1 == max) && (m.2 < len)) {
                    max = m.1;
                    next = Some(*m.0);
                    len = m.2;
                }
            }

            tile_grid[y * tile_grid_width + x] = next;

            matches.remove(matches.iter().position(|e| e.0 == next.unwrap()).unwrap());
        }
    }

    // Working out (0, 0), (0, 1), and (1, 0) are special cases because we might need to flip
    // (0, 0) and (0, 1) if (1, 0) requires us to!

    let mut tile_grid_stripped = Vec::new();
    tile_grid_stripped.resize(tile_grid.len(), 0);

    {
        // Work out (0, 0) and (1, 0) first.
        let mut x = tile_grid[0].unwrap().1;
        let y = &mut tile_grid[tile_grid_width].unwrap().1;

        let mut found = false;

        for _ in 0..3 {
            if let Some(m) = make_match(x, *y, tile_width, true) {
                found = true;
                tile_grid_stripped[0] = x;
                tile_grid_stripped[tile_grid_width] = m;
                break;
            }

            x = rotate_clockwise(x, tile_width);
        }

        assert!(found);
    }

    {
        // Work out (0, 0) and (0, 1) next.
        let x = tile_grid_stripped[0];
        let y = tile_grid[1].unwrap().1;

        if let Some(m) = make_match(x, y, tile_width, false) {
            tile_grid_stripped[1] = m;
        } else {
            // If we get here, it means we need to flip (0, 0) and (1, 0) because
            // we need to match the easterly (0, 1)!
            tile_grid_stripped[0] = flip_horizontal(tile_grid_stripped[0], tile_width);
            tile_grid_stripped[tile_grid_width] =
                flip_horizontal(tile_grid_stripped[tile_grid_width], tile_width);

            tile_grid_stripped[1] =
                make_match(tile_grid_stripped[0], y, tile_width, false).unwrap();
        }
    }

    for y in 0..tile_grid_width {
        for x in 0..tile_grid_width {
            // skip (0, 0), (0, 1), and (1, 0) since we already processed these.
            if (x == 0 && y == 0) || (x == 0 && y == 1) || (x == 1 && y == 0) {
                continue;
            }

            let index = y * tile_grid_width + x;

            let me = tile_grid[index].unwrap().1;

            if x > 0 {
                let prev = tile_grid_stripped[index - 1];

                tile_grid_stripped[index] = make_match(prev, me, tile_width, false).unwrap();
            } else {
                let prev = tile_grid_stripped[index - tile_grid_width];
                tile_grid_stripped[index] = make_match(prev, me, tile_width, true).unwrap();
            }
        }
    }

    let mut bits = BitSet::new();

    let new_tile_width = tile_width - 2;
    let new_width = tile_grid_width * new_tile_width;

    for y in 0..new_width {
        for x in 0..new_width {
            let old_index = ((y / new_tile_width) * tile_grid_width) + (x / new_tile_width);
            let tile = tile_grid_stripped[old_index];

            let row = y % new_tile_width;
            let col = x % new_tile_width;

            let r = get_row(tile, row + 1, tile_width);

            if r.bit(col + 1) {
                let index = y * new_width + x;
                bits.insert(index);
            }
        }
    }

    // Now find the stupid monster!
    let monster_lines = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let mut monster_offsets = Vec::new();

    for (index, line) in monster_lines.iter().enumerate() {
        monster_offsets.append(
            &mut line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(offset, _)| (offset, index))
                .collect(),
        );
    }

    let max = (
        monster_offsets.iter().map(|(x, _)| x).max().unwrap(),
        monster_offsets.iter().map(|(_, y)| y).max().unwrap(),
    );

    for _ in 0..2 {
        for _ in 0..4 {
            let mut num_found = 0;

            for y in 0..new_width {
                // Skip any that can't fit our monster.
                if y + max.1 >= new_width {
                    continue;
                }

                for x in 0..new_width {
                    // Skip any that can't fit our monster.
                    if x + max.0 >= new_width {
                        continue;
                    }

                    let mut found = true;

                    for (offset_x, offset_y) in &monster_offsets {
                        let index = (y + offset_y) * new_width + x + offset_x;

                        if !bits.contains(index) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        num_found += 1;
                    }
                }
            }

            if num_found > 0 {
                let hashes_in_grid = bits.len();
                let monster_hashes = num_found * monster_offsets.len();
                let result = hashes_in_grid - monster_hashes;

                green_ln!(
                    "Day 20, part 02: Number of # not part of stupid monster - {}",
                    result
                );

                return;
            }

            bits = rotate_big_grid(bits, new_width);
        }

        bits = flip_big_grid(bits, new_width);
    }

    panic!("Shouldn't get here (gulp).");
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let mut tile_width = None;

    let mut tiles = Vec::new();

    let mut lines = string.lines();

    loop {
        let tile_line = lines.next();

        if tile_line == None {
            break;
        }

        let tile_id = tile_line
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(':')
            .unwrap();

        let tile_id = tile_id.parse::<i16>().unwrap();

        let mut tile_data: i128 = 0;
        let mut tile_index = 0;

        loop {
            let line = lines.next();

            if line == None {
                break;
            }

            let line = line.unwrap();

            if line.is_empty() {
                break;
            }

            let width = line.chars().count();

            if let Some(tile_width) = tile_width {
                assert_eq!(tile_width, width);
            } else {
                tile_width = Some(width);
            }

            for c in line.chars() {
                match c {
                    '#' => tile_data.set_bit(tile_index, true),
                    '.' => tile_data.set_bit(tile_index, false),
                    _ => panic!("Unknown char!"),
                }

                tile_index += 1;
            }
        }

        let height = tile_index / tile_width.unwrap();

        assert_eq!(tile_width.unwrap(), height);

        tiles.push((tile_id, tile_data));
    }

    let matches = day20_part01(&tiles, tile_width.unwrap());
    day20_part02(&tiles, tile_width.unwrap(), matches);
}
