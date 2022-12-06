use byte_set::ByteSet;

pub fn solve_part1(input: &str) -> usize {
    let binding = input.chars().collect::<Vec<char>>();
    let mut iter = binding.as_slice().windows(4);
    let mut matching: String = String::new();
    for w in iter {

        let mut set = ByteSet::new();
        // I don’t know the standard library well enough
        for i in 0..4 {
            set.insert(w[i] as u8);
        }
        if set.len() == 4 {
            // Ditto
            matching = w.iter().collect::<String>();
            break;
        }
    }
    let mut iter = input.split(&matching);
    iter.next().unwrap().len() + 4
}

pub fn solve_part2(input: &str) -> usize {
    let binding = input.chars().collect::<Vec<char>>();
    let mut iter = binding.as_slice().windows(14);
    let mut matching: String = String::new();
    for w in iter {

        let mut set = ByteSet::new();
        // I don’t know the standard library well enough
        for i in 0..14 {
            set.insert(w[i] as u8);
        }
        if set.len() == 14 {
            // Ditto
            matching = w.iter().collect::<String>();
            break;
        }
    }
    let mut iter = input.split(&matching);
    iter.next().unwrap().len() + 14
}
