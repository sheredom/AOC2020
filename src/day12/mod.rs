#[exec_time]
fn day12_part01(commands: &[(char, i32)]) {
    let mut direction = 90;
    let mut x = 0;
    let mut y = 0;

    for command in commands {
        match command.0 {
            'N' => y -= command.1,
            'S' => y += command.1,
            'W' => x -= command.1,
            'E' => x += command.1,
            'L' => {
                direction -= command.1;
                // Add 360 to force us back into a positive range.
                direction += 360;
                direction %= 360;
            }
            'R' => {
                direction += command.1;
                direction %= 360;
            }
            'F' => match direction {
                0 => x -= command.1,
                180 => x += command.1,
                270 => y -= command.1,
                90 => y += command.1,
                _ => panic!("Bad direction {}", direction),
            },
            _ => panic!("Unknown command {}", command.0),
        }
    }

    let result = x.abs() + y.abs();

    red_ln!("Day 12, part 01: Manhattan distance {}", result);
}

#[exec_time]
fn day12_part02(commands: &[(char, i32)]) {
    let mut x = 0;
    let mut y = 0;

    let mut waypoint_x = 10;
    let mut waypoint_y = -1;

    for command in commands {
        match command.0 {
            'N' => waypoint_y -= command.1,
            'S' => waypoint_y += command.1,
            'W' => waypoint_x -= command.1,
            'E' => waypoint_x += command.1,
            'L' => match command.1 {
                90 => {
                    waypoint_x *= -1;
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                }
                180 => {
                    waypoint_x *= -1;
                    waypoint_y *= -1;
                }
                270 => {
                    waypoint_y *= -1;
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                }
                _ => panic!("Bad rotation {}", command.1),
            },
            'R' => match command.1 {
                90 => {
                    waypoint_y *= -1;
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                }
                180 => {
                    waypoint_x *= -1;
                    waypoint_y *= -1;
                }
                270 => {
                    waypoint_x *= -1;
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                }
                _ => panic!("Bad rotation {}", command.1),
            },
            'F' => {
                for _ in 0..command.1 {
                    x += waypoint_x;
                    y += waypoint_y;
                }
            }
            _ => panic!("Unknown command {}", command.0),
        }
    }

    let result = x.abs() + y.abs();

    green_ln!(
        "Day 12, part 02: Manhattan distance {} + {} = {}",
        x.abs(),
        y.abs(),
        result
    );
}

fn resolve_command(line: &str) -> (char, i32) {
    let (direction, amount) = line.split_at(1);
    (
        direction.chars().next().unwrap(),
        amount.parse::<i32>().unwrap(),
    )
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let iter: Vec<(char, i32)> = string.lines().map(resolve_command).collect();

    day12_part01(&iter);
    day12_part02(&iter);
}
