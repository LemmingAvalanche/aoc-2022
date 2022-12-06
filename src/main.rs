static input: &str = include_str!(
    "/home/kristoffer/programming/advent-of-code-2022/day6-input.txt"
);

fn main() {
    // I just update this function for each problem that I bother to solve,
    // setting the input file path and the function call.
    println!("{}", advent_of_code_2022::day6::solve_part1(&input));
}
