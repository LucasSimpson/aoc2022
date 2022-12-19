
fn as_grid(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines.into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32 )
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    // mark all as invisible
    let mut grid: Vec<Vec<(i32, bool)>> = as_grid(lines).into_iter().map(|v| {
        v.into_iter().map(|h| (h, false)).collect()
    }).collect();

    let x = grid.len();
    let y = grid[0].len();

    for i in 0..x {
        // coming from bottom
        let mut highest: i32 = -1;
        for j in 0..y {
            if grid[i][j].0 > highest {
                highest = grid[i][j].0;
                grid[i][j].1 = true;
            }
        }

        // coming from top
        let mut highest: i32 = -1;
        for j in 0..y {
            if grid[i][y - j - 1].0 > highest {
                highest = grid[i][y - j - 1].0;
                grid[i][y - j - 1].1 = true;
            }
        }
    }

    for j in 0..y {
        // coming from left
        let mut highest: i32 = -1;
        for i in 0..x {
            if grid[i][j].0 > highest {
                highest = grid[i][j].0;
                grid[i][j].1 = true;
            }
        }

        // coming from right
        let mut highest: i32 = -1;
        for i in 0..x {
            if grid[x - i - 1][j].0 > highest {
                highest = grid[x - i - 1][j].0;
                grid[x - i - 1][j].1 = true;
            }
        }
    }

    grid.iter().map(|v| {
        v.iter().map(|(_, visible)| {
            if *visible {
                1
            } else {
                0
            }
        }).sum::<u32>()
    }).sum()
}

fn scenic_score_at(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let x = grid.len();
    let y = grid[0].len();

    let mut scores: Vec<i32> = vec![];
    let t = grid[i][j];

    // looking right
    let mut k = 0;
    loop {
        k += 1;
        if i + k >= x {
            k -= 1;
            break
        }
        if grid[i + k][j] >= t {
            break
        }
    }
    scores.push(k as i32);

    // looking left
    let mut k = 0;
    loop {
        k += 1;
        if k > i {
            k -= 1;
            break
        }
        if grid[i - k][j] >= t {
            break
        }
    }
    scores.push(k as i32);

    // looking up
    let mut k = 0;
    loop {
        k += 1;
        if j + k >= y {
            k -= 1;
            break
        }
        if grid[i][j + k] >= t {
            break
        }
    }
    scores.push(k as i32);

    // looking down
    let mut k = 0;
    loop {
        k += 1;
        if k > j {
            k -= 1;
            break
        }
        if grid[i][j - k] >= t {
            break
        }
    }
    scores.push(k as i32);


    scores.into_iter().reduce(|acc, v| acc * v ).unwrap()
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    let grid = as_grid(lines);

    let mut highest = 0;
    for (i, v) in grid.clone().into_iter().enumerate() {
        for (j, _) in v.into_iter().enumerate() {
            let ss = scenic_score_at(&grid, i, j);
            if ss > highest {
                highest = ss;
            }
        }
    }

    highest as u32
}

#[cfg(test)]
mod tests {
    use crate::day8::day8::{as_grid, scenic_score_at, solve_p1, solve_p2};

    #[test]
    fn test_scenic_score_at() {
        let lines: Vec<String> = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];
        let grid = as_grid(lines);
        assert_eq!(scenic_score_at(&grid, 1, 2), 4);
        assert_eq!(scenic_score_at(&grid, 3, 2), 8);
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 21)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 8)
    }
}

