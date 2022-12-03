use byte_set::ByteSet;

// as usual
pub fn part1_solve(input: &str) -> u32 {
    input.lines().map(|l| score_rucksack(l)).sum()
}

fn score_rucksack(rucksack: &str) -> u32 {
    let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
    let comp1_set = str_to_set(comp1);
    let comp2_set = str_to_set(comp2);
    let intersection = comp1_set.intersection(comp2_set);
    // only one
    let common = intersection.into_iter().nth(0).expect("bad input?");
    // upper-case
    if common <= 90 {
        common as u32 - 38
    } else {
        common as u32 - 96
    }
}

fn str_to_set(s: &str) -> byte_set::ByteSet {
    let mut set = ByteSet::new();
    for b in s.as_bytes() {
        set.insert(*b);
    }
    set
}
