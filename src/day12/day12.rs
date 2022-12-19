use std::collections::VecDeque;

fn can_move(from: (usize, usize), to: (usize, usize), heights: &Vec<Vec<u32>>) -> bool {
    heights[from.0][from.1] + 1 >= heights[to.0][to.1]
}

fn calculate_steps(start: (usize, usize), heights: &Vec<Vec<u32>>, steps: &mut Vec<Vec<Option<u32>>>) {
    let mut to_check: VecDeque<((usize, usize), u32)> = VecDeque::from([(start, 0)]);

    loop {
        let (pos, current_steps) = match to_check.pop_front() {
            None => {
                return
            },
            Some(pos) => pos,
        };

        let lowest = match steps[pos.0][pos.1] {
            None => current_steps,
            Some(prev) => {
                if prev <= current_steps {
                    continue
                } else {
                    current_steps
                }
            }
        };
        steps[pos.0][pos.1] = Some(lowest);

        vec![
            (pos.0 as i32 - 1, pos.1 as i32),
            (pos.0 as i32 + 1, pos.1 as i32),
            (pos.0 as i32, pos.1 as i32 - 1),
            (pos.0 as i32, pos.1 as i32 + 1),
        ].into_iter()
            .filter(|(x, y)| {
                *x >= 0 && *x < heights.len() as i32 && *y >= 0 && *y < heights[0].len() as i32
            })
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|new_pos| {
                can_move(pos, *new_pos, heights)
            })
            .for_each(|new_pos| {
                to_check.push_back((new_pos, current_steps + 1));
            });
    }
}


pub fn solve_p1(lines: Vec<String>) -> u32 {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let heights: Vec<Vec<u32>> = lines.into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        'S' => {
                            start = (i, j);
                            0
                        }
                        'E' => {
                            end = (i, j);
                            25
                        }
                        _ => c as u32 - 97
                    }
                }).collect::<Vec<u32>>()
        })
        .collect();

    let mut steps: Vec<Vec<Option<u32>>> = heights.iter().map(|l| {
        l.iter().map(|_| None).collect()
    }).collect();

    calculate_steps(start, &heights, &mut steps);
    steps[end.0][end.1].unwrap()
}


pub fn solve_p2(lines: Vec<String>) -> u32 {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let heights: Vec<Vec<u32>> = lines.into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        'S' => {
                            start = (i, j);
                            0
                        }
                        'E' => {
                            end = (i, j);
                            25
                        }
                        _ => c as u32 - 97
                    }
                }).collect::<Vec<u32>>()
        })
        .map(|h| h.iter().map(|x| 25 - x ).collect() ) // invert
        .collect();

    let mut steps: Vec<Vec<Option<u32>>> = heights.iter().map(|l| {
        l.iter().map(|_| None).collect()
    }).collect();

    // start at the end
    calculate_steps(end, &heights, &mut steps);

    let mut min_steps :Vec<u32> = heights.iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, h)| {
                    (i, j)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .filter(|(x, y)| heights[*x][*y] == 25)
        .map(|(x, y)| steps[x][y])
        .filter(Option::is_some)
        .map(|o| o.unwrap())
        .collect();

    min_steps.sort();
    min_steps[0]
}

#[cfg(test)]
mod tests {
    use crate::day12::day12::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "Sabqponm".to_owned(),
            "abcryxxl".to_owned(),
            "accszExk".to_owned(),
            "acctuvwj".to_owned(),
            "abdefghi".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 31)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "Sabqponm".to_owned(),
            "abcryxxl".to_owned(),
            "accszExk".to_owned(),
            "acctuvwj".to_owned(),
            "abdefghi".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 29)
    }
}

