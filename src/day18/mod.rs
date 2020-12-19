// Follow Nathan Reed's explanation of the Shunting Yard algorithm
// http://reedbeta.com/blog/the-shunting-yard-algorithm/
fn shunting_yard(mut expression: &str, is_add_higher_precedence: bool) -> i64 {
    let mut operand_stack = Vec::new();
    let mut operator_stack = Vec::new();

    let apply = |operator, x: i64, y: i64| match operator {
        '*' => x * y,
        '+' => x + y,
        _ => panic!("Unknown operator!"),
    };

    loop {
        // Remove any pesky whitespace.
        expression = expression.trim_start();

        if expression.is_empty() {
            break;
        }

        if let Some(stripped) = expression.strip_prefix(|c| match c {
            '*' => true,
            '+' => true,
            _ => false,
        }) {
            let new_operator = expression.chars().next().unwrap();

            // In the 1st part we want to have left-to-right precedence always, but in the 2nd
            // part we want add to have a higher precedence than mul. So we want to process the
            // operator stack here either if a) we don't have add having a higher precedence, or
            // b) the operator is a multiply and thus we should flush the stack.
            if !is_add_higher_precedence || new_operator == '*' {
                while let Some(operator) = operator_stack.pop() {
                    if operator == '(' {
                        // We don't want to remove any parenthesis here!
                        operator_stack.push(operator);
                        break;
                    } else {
                        let y = operand_stack.pop().unwrap();
                        let x = operand_stack.pop().unwrap();
                        operand_stack.push(apply(operator, x, y));
                    }
                }
            }

            operator_stack.push(new_operator);
            expression = stripped;
        } else if let Some(stripped) = expression.strip_prefix('(') {
            operator_stack.push(expression.chars().next().unwrap());
            expression = stripped;
        } else if let Some(stripped) = expression.strip_prefix(')') {
            while let Some(operator) = operator_stack.pop() {
                if operator == '(' {
                    break;
                } else {
                    let y = operand_stack.pop().unwrap();
                    let x = operand_stack.pop().unwrap();
                    operand_stack.push(apply(operator, x, y));
                }
            }
            expression = stripped;
        } else {
            // The unwrap_or here just catches the case where we have an integer
            // at the end of the expression, and so we want to capture the entire
            // remaining expression.
            let split = expression
                .find(|c: char| !c.is_numeric())
                .unwrap_or_else(|| expression.len());

            let (number, stripped) = expression.split_at(split);
            operand_stack.push(number.parse::<i64>().unwrap());
            expression = stripped;
        }
    }

    while let Some(operator) = operator_stack.pop() {
        let y = operand_stack.pop().unwrap();
        let x = operand_stack.pop().unwrap();
        operand_stack.push(apply(operator, x, y));
    }

    operand_stack.pop().unwrap()
}

#[exec_time]
fn day18_part01(string: &str) {
    let mut total = 0;
    for line in string.lines() {
        total += shunting_yard(&line, false);
    }

    red_ln!("Day 18, part 01: Sum of all expressions {} ", total);
}

#[exec_time]
fn day18_part02(string: &str) {
    let mut total = 0;
    for line in string.lines() {
        total += shunting_yard(&line, true);
    }

    green_ln!(
        "Day 18, part 02: Sum of all expression (with add higher precedence) {} ",
        total
    );
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day18_part01(&string);
    day18_part02(&string);
}
