use std::cmp::min;
use std::collections::HashSet;
use std::hash::Hash;
use regex::Regex;

fn parse_lines(lines: Vec<String>) -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    lines.into_iter()
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            (
                (&captures[1]).parse::<i32>().unwrap(),
                (&captures[2]).parse::<i32>().unwrap(),
                (&captures[3]).parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn value_for(b: bool) -> i32 {
    match b {
        true => 0,
        false => 1,
    }
}

fn add(&(x, y, z): &(i32, i32, i32), &(i, j, k): &(i32, i32, i32)) -> (i32, i32, i32) {
    (x + i, y + j, z + k)
}

fn get_offsets() -> Vec<(i32, i32, i32)> {
    Vec::from([
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ])
}

fn fill_grid(lines: Vec<String>) -> HashSet<(i32, i32, i32)> {
    let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();

    for pos in parse_lines(lines) {
        grid.insert(pos);
    }
    grid
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    let grid = fill_grid(lines);
    let offsets = get_offsets();

    let mut faces = 0;
    for pos in grid.iter() {
        for offset in offsets.iter() {
            faces += value_for(grid.contains(&add(pos, offset)));
        }
    }

    faces as u32
}

fn flood(
    outside: &mut HashSet<(i32, i32, i32)>,
    grid: &HashSet<(i32, i32, i32)>,
    pos: (i32, i32, i32),
    low_x: i32,
    high_x: i32,
    low_y: i32,
    high_y: i32,
    low_z: i32,
    high_z: i32,
) {
    // if out of bounds, quit
    if pos.0 < low_x - 1 || pos.0 > high_x + 1 ||
        pos.1 < low_y - 1 || pos.1 > high_y + 1 ||
        pos.2 < low_z - 1 || pos.2 > high_z + 1 {
        return
    }

    // if inside grid, quit
    if grid.contains(&pos) {
        return
    }

    // if already visited, quit
    if !outside.insert(pos) {
        return
    }

    // check all neighbours
    for offset in get_offsets().iter() {
        flood(outside, grid, add(&pos, offset), low_x, high_x, low_y, high_y, low_z, high_z);
    }
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    let grid = fill_grid(lines);

    // find bounds
    let low_x = grid.iter().map(|(x, _, _)| x).min().unwrap().to_owned();
    let high_x = grid.iter().map(|(x, _, _)| x).max().unwrap().to_owned();
    let low_y = grid.iter().map(|(_, y, _)| y).min().unwrap().to_owned();
    let high_y = grid.iter().map(|(_, y, _)| y).max().unwrap().to_owned();
    let low_z = grid.iter().map(|(_, _, z)| z).min().unwrap().to_owned();
    let high_z = grid.iter().map(|(_, _, z)| z).max().unwrap().to_owned();

    // flood
    let mut outside: HashSet<(i32, i32, i32)> = HashSet::new();
    flood(&mut outside, &grid, (low_x - 1, low_y - 1, low_z - 1), low_x, high_x, low_y, high_y, low_z, high_z);

    let offsets = get_offsets();
    let mut faces = 0;
    for pos in grid.iter() {
        for offset in offsets.iter() {
            let new_pos = add(pos, offset);
            if !grid.contains(&new_pos) {
                if outside.contains(&new_pos) {
                    faces += 1;
                }
            }
        }
    }

    faces as u32
}

#[cfg(test)]
mod tests {
    use crate::day18::day18::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1_small() {
        let lines: Vec<String> = vec![
            "1,1,1".to_owned(),
            "2,1,1".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 10)
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "2,2,2".to_owned(),
            "1,2,2".to_owned(),
            "3,2,2".to_owned(),
            "2,1,2".to_owned(),
            "2,3,2".to_owned(),
            "2,2,1".to_owned(),
            "2,2,3".to_owned(),
            "2,2,4".to_owned(),
            "2,2,6".to_owned(),
            "1,2,5".to_owned(),
            "3,2,5".to_owned(),
            "2,1,5".to_owned(),
            "2,3,5".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 64)
    }

    #[test]
    fn test_solve_p2_small() {
        let lines: Vec<String> = vec![
            "1,1,1".to_owned(),
            "2,1,1".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 10)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "2,2,2".to_owned(),
            "1,2,2".to_owned(),
            "3,2,2".to_owned(),
            "2,1,2".to_owned(),
            "2,3,2".to_owned(),
            "2,2,1".to_owned(),
            "2,2,3".to_owned(),
            "2,2,4".to_owned(),
            "2,2,6".to_owned(),
            "1,2,5".to_owned(),
            "3,2,5".to_owned(),
            "2,1,5".to_owned(),
            "2,3,5".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 58)
    }
}

