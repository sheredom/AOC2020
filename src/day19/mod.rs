use itertools::Itertools;

fn matches<'a>(rules: &[Rule], input: &'a str, next: usize) -> Vec<&'a str> {
    let lambda = |vec: &Vec<usize>| -> Vec<&str> {
        let mut work = Vec::new();
        work.push(input);

        for next in vec {
            let mut next_work = Vec::new();

            for input in work {
                next_work.append(&mut matches(rules, input, *next));
            }

            work = next_work;
        }

        work
    };

    // If the input is empty we push an empty string because we did find a match!
    if input.is_empty() {
        return vec![];
    }

    match &rules[next] {
        Rule::Next(vec) => lambda(vec),
        Rule::Choice(left, right) => {
            let mut ret = lambda(left);
            ret.append(&mut lambda(right));
            ret
        }
        Rule::End(c) => {
            let mut ret = Vec::new();

            // If the input is empty we push an empty string because we did find a match!
            if input.is_empty() {
                let mut ret = Vec::new();
                ret.push("");
                return ret;
            }

            if *c == input.chars().next().unwrap() {
                ret.push(&input[1..]);
            }

            ret
        }
        _ => panic!("Unhandled rule"),
    }
}

fn contains<'a>(rules: &[Rule], input: &'a str) -> bool {
    matches(rules, input, 0)
        .iter()
        .filter(|item| item.is_empty())
        .count()
        != 0
}

#[exec_time]
fn day19_part01(rules: &[Rule], inputs: &[&str]) {
    let total = inputs
        .iter()
        .filter(|input| contains(&rules, input))
        .count();

    red_ln!("Day 19, part 01: Messages that match rule 0 - {}", total);
}

#[exec_time]
fn day19_part02(original: Vec<Rule>, inputs: &[&str]) {
    let mut rules = original;

    // 8: 42 | 42 8
    rules[8] = Rule::Choice(vec![42], vec![42, 8]);

    // 11: 42 31 | 42 11 31
    rules[11] = Rule::Choice(vec![42, 31], vec![42, 11, 31]);

    let total = inputs
        .iter()
        .filter(|input| contains(&rules, input))
        .count();

    green_ln!(
        "Day 19, part 02: Messages that match rule 0 with loops - {}",
        total
    );
}

#[derive(Clone, Debug)]
enum Rule {
    Next(Vec<usize>),
    Choice(Vec<usize>, Vec<usize>),
    End(char),
    Shite,
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    let mut rules = Vec::new();

    for rule in string.lines().take_while(|line| !line.is_empty()) {
        let (id, rule) = rule.splitn(2, ':').next_tuple().unwrap();
        let id = id.parse::<usize>().unwrap();
        rules.resize(std::cmp::max(rules.len(), id + 1), Rule::Shite);

        if let Some((left, right)) = rule.splitn(2, '|').next_tuple() {
            let left = left
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            let right = right
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            rules[id] = Rule::Choice(left, right);
        } else if rule.trim_start().starts_with('"') {
            rules[id] = Rule::End(rule.trim_start().chars().nth(1).unwrap());
        } else {
            let next = rule
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            rules[id] = Rule::Next(next);
        }
    }

    // Skip 1 because we want to also skip the empty line.
    let inputs: Vec<_> = string
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();

    day19_part01(&rules, &inputs);
    day19_part02(rules, &inputs);
}
