use regex::Regex;

#[derive(Debug)]
struct Instructions {
    movex: usize,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<char>>;

pub fn solve_part1(input: &str) -> String {
    let (instructions, mut stacks) = to_data(&input);

    for Instructions { movex, from, to } in instructions {
        for _ in 1..=movex {
            let from_stack = stacks.get_mut(from).expect("from_stack failed");
            let p = from_stack.pop().expect("pop failed");
            let to_stack = stacks.get_mut(to).expect("to_stack failed");
            to_stack.push(p);
        }
    }

    let mut answer = String::new();
    for s in &mut stacks {
        if !s.is_empty() {
            answer.push(s.pop().unwrap());
        }
    }

    answer
}

pub fn solve_part2(input: &str) -> String {
    let (instructions, mut stacks) = to_data(&input);

    for Instructions { movex, from, to } in instructions {
        let from_stack = stacks.get_mut(from).expect("from_stack failed");
        let drained: Vec<_> = from_stack
            .drain(from_stack.len() - movex..from_stack.len())
            .collect();
        drop(from_stack);
        let to_stack = stacks.get_mut(to).expect("to_stack failed");
        to_stack.extend_from_slice(&drained);
    }

    let mut answer = String::new();
    for s in &mut stacks {
        if !s.is_empty() {
            answer.push(s.pop().unwrap());
        }
    }

    answer
}

fn to_data(input: &str) -> (Vec<Instructions>, Stacks) {
    let (krates, instructions) = input.split_once("\n\n").unwrap();

    let width = krates.split_once("\n").unwrap().0.len();
    let height = krates.len() / width;
    let mut transposed = vec!['1'; width * height];
    // transpose so that stacks can be read left-to-right… like a normal
    // textual input
    // Next we will split the `&str` on each stack
    transpose::transpose(
        &krates
            .chars()
            // get rid of brackets
            .map(|c| if c == '[' || c == ']' { ' ' } else { c })
            // newlines mess up squareness
            .filter(|c| *c != '\n')
            .collect::<Vec<char>>(),
        &mut transposed,
        width,
        height,
    );

    // borrow checker
    let binding: String = transposed
        .iter()
        .rev()
        .into_iter()
        .clone()
        .collect::<String>();
    // Now we can read each stack by iterating through the string, e.g.
    // `1ZN`
    let better_strs = binding.split_whitespace().collect::<Vec<&str>>();

    // enough space; 0 index is a dummy vec
    let mut stacks = vec![Vec::new(); 50];
    for string in &better_strs {
        let mut iter = string.chars();
        let index: usize = iter
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();
        stacks[index] = iter.collect();
    }

    // and now instructions
    let instructions_data = instructions
        .lines()
        .map(|l| parse_instructions(l))
        .collect::<Vec<Instructions>>();
    (instructions_data, stacks)
}

fn parse_instructions(line: &str) -> Instructions {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    if let Some(raw_assgns) = RE.captures(line) {
        Instructions {
            movex: parse_usize(&raw_assgns[1]),
            from: parse_usize(&raw_assgns[2]),
            to: parse_usize(&raw_assgns[3]),
        }
    } else {
        panic!("bad input or parse. Input: {line}");
    }
}

fn parse_usize(i: &str) -> usize {
    i.parse::<usize>().expect("not an integer")
}
