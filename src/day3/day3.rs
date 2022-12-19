use std::collections::HashSet;
use std::fs;


fn get_lines(day: u32) -> Vec<String>  {
// Read the file into a string
    let file_contents = fs::read_to_string(format!("src/day{day}/input.txt")).expect("Failed to read file");

    // Split the string on newline characters to get a vector of strings
    return file_contents.split("\n").map(|l| l.trim().to_string() ).collect()
}

pub fn run() {
    let lines = get_lines(3);

    print!("{}", solve_p2(lines));
}

fn solve_p1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(to_priorities)
        .map(to_rucksacks)
        .map(find_common)
        .sum()
}

fn solve_p2(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(to_priorities)
        .map(to_rucksacks)
        .map(to_set)
        .collect::<Vec<HashSet<u32>>>()
        .chunks(3)
        .map(|x| x.to_vec())
        // .collect::<Vec<HashSet<u32>>>()
        .map(find_common_badge)
        .sum()
}

fn map_char(c: char) -> u32 {
    let n = c as u32;
    if n >= 97 {
        n - 96
    } else {
        n - 64 + 26
    }
}

fn to_priorities(data: &String) -> Vec<u32> {
    data.chars().map(map_char).collect()
}

fn to_rucksacks(priorities: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let mid = priorities.len() / 2;
    let (first_half, second_half) = priorities.split_at(mid);

    (first_half.to_vec(), second_half.to_vec())
}

fn to_set(rucksack: (Vec<u32>, Vec<u32>)) -> HashSet<u32> {
    HashSet::from_iter(
        rucksack.0.into_iter().chain(rucksack.1.into_iter())
    )
}

fn find_common(rucksack: (Vec<u32>, Vec<u32>)) -> u32 {
    for x in rucksack.0.into_iter() {
        if rucksack.1.contains(&x) {
            return x;
        }
    }
    0
}

fn find_common_badge(group: Vec<HashSet<u32>>) -> u32 {
    *group
        .into_iter()
        .reduce(|x, y| x.intersection(&y).map(|x| *x ).collect() )
        .unwrap()
        .iter()
        .take(1)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day3::day3::{map_char, solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(solve_p1(lines), 157)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(solve_p2(lines), 70)
    }

    #[test]
    fn test_map_char() {
        assert_eq!(map_char('a'), 1);
        assert_eq!(map_char('z'), 26);
        assert_eq!(map_char('A'), 27);
        assert_eq!(map_char('Z'), 52);
    }
}
