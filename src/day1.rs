pub fn part1_max_calories(input: &str) -> i64 {
    input.split("\n\n")
        .map(|elf| { elf
                .lines()
                .map(|s| s.parse::<i64>().expect("not an integer"))
                     // NOTE Why can’t I use `sum(…)`?
                .fold(0, |acc, len| acc + len )
        })
        .max()
        .expect("Empty input?")
}
