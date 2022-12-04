use regex::Regex;

type Pair = (u8, u8);

fn fully_contains(first: Pair, second: Pair) -> bool {
    let (start1, end1) = first;
    let (start2, end2) = second;
    start1 <= start2 && end1 >= end2
}

fn overlaps(first: Pair, second: Pair) -> bool {
    let (start1, end1) = first;
    let (start2, end2) = second;
    (start1 <= start2 && end1 >= end2) || (end1 >= start2 && start1 <= start2)
}

pub fn part1_solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_to_assgns(l))
        .filter(|(pair1, pair2)| fully_contains(*pair1, *pair2) || fully_contains(*pair2, *pair1))
        .count()
}

pub fn part2_solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_to_assgns(l))
        .filter(|(pair1, pair2)| overlaps(*pair1, *pair2) || overlaps(*pair2, *pair1))
        .count()
}

/// parse one line of input to two pairs
fn parse_to_assgns(line: &str) -> (Pair, Pair) {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    if let Some(raw_assgns) = RE.captures(line) {
        (
            (parse_u8(&raw_assgns[1]), parse_u8(&raw_assgns[2])),
            (parse_u8(&raw_assgns[3]), parse_u8(&raw_assgns[4])),
        )
    } else {
        panic!("bad input or parse. Input: {line}");
    }
}

fn parse_u8(i: &str) -> u8 {
    i.parse::<u8>().expect("not an integer")
}
