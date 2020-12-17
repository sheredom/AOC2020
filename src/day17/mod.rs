use bit_set::BitSet;

fn get_index(
    (x_size, y_size, z_size, _): (usize, usize, usize, usize),
    (x, y, z, w): (usize, usize, usize, usize),
) -> usize {
    w * x_size * y_size * z_size + z * x_size * y_size + y * x_size + x
}

fn insert(
    problem_space: &mut BitSet,
    (x_size, y_size, z_size, w_size): (usize, usize, usize, usize),
    (x, y, z, w): (usize, usize, usize, usize),
) {
    problem_space.insert(get_index((x_size, y_size, z_size, w_size), (x, y, z, w)));
}

fn remove(
    problem_space: &mut BitSet,
    (x_size, y_size, z_size, w_size): (usize, usize, usize, usize),
    (x, y, z, w): (usize, usize, usize, usize),
) {
    problem_space.remove(get_index((x_size, y_size, z_size, w_size), (x, y, z, w)));
}

fn contains(
    problem_space: &BitSet,
    (x_size, y_size, z_size, w_size): (usize, usize, usize, usize),
    (x, y, z, w): (usize, usize, usize, usize),
) -> bool {
    problem_space.contains(get_index((x_size, y_size, z_size, w_size), (x, y, z, w)))
}

fn get_range(v_size: usize, v: usize) -> std::ops::Range<usize> {
    let min = if v > 0 { v - 1 } else { v };
    let max = if v < (v_size - 1) { v + 2 } else { v + 1 };

    min..max
}

fn query_neighbours(
    problem_space: &BitSet,
    (x_size, y_size, z_size, w_size): (usize, usize, usize, usize),
    (x, y, z, w): (usize, usize, usize, usize),
) -> usize {
    let mut total = 0;

    for ow in get_range(w_size, w) {
        for oz in get_range(z_size, z) {
            for oy in get_range(y_size, y) {
                for ox in get_range(x_size, x) {
                    // Skip ourselves.
                    if (x, y, z, w) == (ox, oy, oz, ow) {
                        continue;
                    }

                    total += if contains(
                        problem_space,
                        (x_size, y_size, z_size, w_size),
                        (ox, oy, oz, ow),
                    ) {
                        1
                    } else {
                        0
                    };
                }
            }
        }
    }

    total
}

fn solve(
    problem_space: &mut BitSet,
    (x_size, y_size, z_size, w_size): (usize, usize, usize, usize),
) -> usize {
    let mut to_removes = Vec::new();
    let mut to_inserts = Vec::new();

    for _ in 0..6 {
        for w in 0..w_size {
            for z in 0..z_size {
                for y in 0..y_size {
                    for x in 0..x_size {
                        let active_neighbours = query_neighbours(
                            &problem_space,
                            (x_size, y_size, z_size, w_size),
                            (x, y, z, w),
                        );

                        if contains(
                            &problem_space,
                            (x_size, y_size, z_size, w_size),
                            (x, y, z, w),
                        ) {
                            match active_neighbours {
                                2 => (),
                                3 => (),
                                _ => to_removes.push((x, y, z, w)),
                            }
                        } else if 3 == active_neighbours {
                            to_inserts.push((x, y, z, w));
                        }
                    }
                }
            }
        }

        for to_remove in &to_removes {
            remove(problem_space, (x_size, y_size, z_size, w_size), *to_remove);
        }

        for to_insert in &to_inserts {
            insert(problem_space, (x_size, y_size, z_size, w_size), *to_insert);
        }

        to_removes.clear();
        to_inserts.clear();
    }

    problem_space.iter().count()
}

#[exec_time]
fn day17_part01(original: BitSet, (x_size, y_size, z_size, w_size): (usize, usize, usize, usize)) {
    let mut problem_space = original;

    let total = solve(&mut problem_space, (x_size, y_size, z_size, w_size));

    red_ln!(
        "Day 17, part 01: 3d surviving cubes after 6 cycles {} ",
        total
    );
}

#[exec_time]
fn day17_part02(original: BitSet, (x_size, y_size, z_size, w_size): (usize, usize, usize, usize)) {
    let mut problem_space = original;

    let total = solve(&mut problem_space, (x_size, y_size, z_size, w_size));

    green_ln!(
        "Day 17, part 02: 4d surviving cubes after 6 cycles {} ",
        total
    );
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    // Our 2d plane will _maximally grow_ by a constant factor in each dimension
    // for each cycle calculated. In the first iteration we add an additional
    // boundary in the x/y plane, and two additional planes in the z dimension.
    // We then repeat this again for each cycle we process. Since we are
    // processing 6 cycles, we can calculate the max space needed now.

    let steps = 6;

    let x_size = string.lines().next().unwrap().len();
    assert!(string.lines().all(|line| x_size == line.len()));
    let y_size = string.lines().count();

    // 2 * steps because we grow in each direction.
    let x_size = x_size + 2 * steps;
    let y_size = y_size + 2 * steps;

    // 1 + 2 * steps because we have a single 2d plane source + steps to grow in
    // each direction in 3d.
    let z_size = 1 + 2 * steps;

    // 1 + 2 * steps because we have a single 2d plane source + steps to grow in
    // each direction in 4d.
    let w_size = 1 + 2 * steps;

    let mut problem_space_3d = BitSet::with_capacity(x_size * y_size * z_size);
    let mut problem_space_4d = BitSet::with_capacity(x_size * y_size * z_size * w_size);

    // Now we need to insert our 2d plane into the 3d space we've created.
    for (y, line) in string.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    insert(
                        &mut problem_space_3d,
                        (x_size, y_size, z_size, 1),
                        (x + steps, y + steps, steps, 0),
                    );
                    insert(
                        &mut problem_space_4d,
                        (x_size, y_size, z_size, w_size),
                        (x + steps, y + steps, steps, steps),
                    );
                }
                '.' => (),
                _ => panic!("Unknown char"),
            }
        }
    }

    day17_part01(problem_space_3d, (x_size, y_size, z_size, 1));
    day17_part02(problem_space_4d, (x_size, y_size, z_size, w_size));
}
