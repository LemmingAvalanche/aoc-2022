use std::fs;

use advent_of_code_2022::day3::*;

fn main() {
    // I just update this function for each problem that I bother to solve,
    // setting the input file path and the function call.

    let input =
        fs::read_to_string("/home/kristoffer/programming/advent-of-code-2022/day3-input.txt")
            .unwrap();
    println!("{}", part1_solve(&input));
}
