use byte_set::ByteSet;

// as usual
pub fn part1_solve(input: &str) -> u32 {
    input.lines().map(|l| score_rucksack_part1(l)).sum()
}

pub fn part2_solve(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| score_rucksacks_part2(lines[0], lines[1], lines[2]))
        .sum()
}

fn score_rucksack_part1(rucksack: &str) -> u32 {
    let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
    let comp1_set = ByteSet::from(comp1);
    let comp2_set = ByteSet::from(comp2);
    let intersection = comp1_set.intersection(comp2_set);
    score(intersection)
}

fn score_rucksacks_part2(r1: &str, r2: &str, r3: &str) -> u32 {
    let rset1 = ByteSet::from(r1);
    let rset2 = ByteSet::from(r2);
    let rset3 = ByteSet::from(r3);
    let intersection = rset1.intersection(rset2).intersection(rset3);
    score(intersection)
}

fn score(set: ByteSet) -> u32 {
    // only one
    let common = set.into_iter().nth(0).expect("bad input?");
    // upper-case
    if common <= 90 {
        common as u32 - 38
    } else {
        common as u32 - 96
    }
}
