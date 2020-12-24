use bit_set::BitSet;

#[exec_time]
fn day21_part01(string: String) -> (BitSet, usize) {
    // This is just an arbitrary number that happened to work. I couldn't be bothered to work out a better number.
    let max_line_len = 150;

    let mut hex_grid = BitSet::new();

    // The maximum width in any direction from the origin is 2 * the maximum line length. 1 is just for the centre tile.
    let width = 1 + 2 * max_line_len;

    let origin = 1 + max_line_len;

    for line in string.lines() {
        let mut line = line;
        let mut position = (origin, origin);

        // Using odd-q vertical layout with flat topped grids.
        loop {
            if let Some(ne) = line.strip_prefix("ne") {
                position.0 += 1;
                position.1 -= 1;
                line = ne;
            } else if let Some(nw) = line.strip_prefix("nw") {
                position.1 -= 1;
                line = nw;
            } else if let Some(se) = line.strip_prefix("se") {
                position.1 += 1;
                line = se;
            } else if let Some(sw) = line.strip_prefix("sw") {
                position.0 -= 1;
                position.1 += 1;
                line = sw;
            } else if let Some(e) = line.strip_prefix("e") {
                position.0 += 1;
                line = e;
            } else if let Some(w) = line.strip_prefix("w") {
                position.0 -= 1;
                line = w;
            } else {
                break;
            }
        }

        let index = position.1 * width + position.0;

        if hex_grid.contains(index) {
            hex_grid.remove(index);
        } else {
            hex_grid.insert(index);
        }
    }

    red_ln!("Day 24, part 01: Black tiles {}", hex_grid.len());

    (hex_grid, width)
}

#[exec_time]
fn day21_part02(original: BitSet, width: usize) {
    let mut hex_grid = original;

    let days = 100;

    let mut pending = Vec::new();

    let width = width as isize;

    for _ in 0..days {
        for tile in 0..(width * width) {
            let (q, r) = (tile % width, tile / width);

            // If the tile is on edge, we skip it.
            if (q == 0) || (q == (width - 1)) || (r == 0) || (r == (width - 1)) {
                continue;
            }

            let neighbours = [(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)];

            let mut black_neigbhours = 0;

            for neighbour in neighbours.iter() {
                let neighbour_qr = ((q + neighbour.0), (r + neighbour.1));
                let neighbour_index = neighbour_qr.1 * width + neighbour_qr.0;

                if hex_grid.contains(neighbour_index as usize) {
                    black_neigbhours += 1;
                }
            }

            // Am I black?
            if hex_grid.contains(tile as usize) {
                if (black_neigbhours == 0) || (black_neigbhours > 2) {
                    pending.push(tile);
                }
            } else {
                if black_neigbhours == 2 {
                    pending.push(tile);
                }
            }
        }

        for p in &pending {
            let p = *p as usize;
            // We are flipping our state in the grid here, if we were set, we remove ourselves.
            if hex_grid.contains(p) {
                hex_grid.remove(p);
            } else {
                hex_grid.insert(p);
            }
        }

        pending.clear();
    }

    green_ln!("Day 24, part 02: Tiles after 100 days {}", hex_grid.len());
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    // My hex grid is https://www.redblobgames.com/grids/hexagons/
    let (hex_grid, width) = day21_part01(string);

    day21_part02(hex_grid, width);
}
