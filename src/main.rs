use std::fs;
mod day16;

use crate::day16::day16::{solve_p1, solve_p2};

fn get_lines(day: u32) -> Vec<String>  {
    let file_contents = fs::read_to_string(format!("src/day{day}/input.txt")).expect("Failed to read file");
    return file_contents
        .split("\n")
        .map(|l| l.trim().to_string() )
        .collect()
}


fn main() {
    let lines = get_lines(16);
    println!("part 1: {}", solve_p1(lines.clone())); // 6479 => too high
    println!("part 2: {}", solve_p2(lines));
}

