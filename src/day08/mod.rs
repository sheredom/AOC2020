use bit_set::BitSet;
use itertools::Itertools;

fn interpret(vec: &[&str], maybe_modify: Option<usize>) -> (bool, i32) {
    let mut visited = BitSet::with_capacity(vec.len());

    let mut index = 0;
    let mut acc = 0;

    loop {
        if (index as usize) >= vec.len() {
            return (true, acc);
        }

        if visited.contains(index as usize) {
            return (false, acc);
        }

        visited.insert(index as usize);

        let (mut inst, cnst_str) = vec[index as usize].splitn(2, ' ').next_tuple().unwrap();

        if let Some(modify) = maybe_modify {
            if modify == index as usize {
                match inst {
                    "acc" => return (false, -1),
                    "jmp" => inst = "nop",
                    "nop" => inst = "jmp",
                    _ => panic!("Unknown instruction!"),
                }
            }
        }

        let cnst = cnst_str.parse::<i32>().unwrap();

        match inst {
            "acc" => acc += cnst,
            "jmp" => {
                index += cnst;
                continue;
            }
            "nop" => (),
            _ => panic!("Unknown instruction!"),
        }

        index += 1;
    }
}

#[exec_time]
fn day08_part01(vec: &[&str]) {
    let (_, acc) = interpret(vec, None);
    red_ln!("Day 08, part 01: Accumulator at second visit {}", acc);
}

#[exec_time]
fn day08_part02(vec: &[&str]) {
    let mut acc = 0;

    for modify in 0..vec.len() {
        let (normal_exit, inner_acc) = interpret(vec, Some(modify));

        acc = inner_acc;

        if normal_exit {
            break;
        }
    }

    green_ln!(
        "Day 08, part 02: Accumulator at modified instruction stream {}",
        acc
    );
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();
    let vec: Vec<&str> = string.lines().collect();

    day08_part01(&vec);
    day08_part02(&vec);
}
