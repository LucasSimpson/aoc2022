use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn do_move(&mut self, dir: Direction) {
        match dir {
            Direction::Right => {
                self.x += 1;
            },
            Direction::Left => {
                self.x -= 1;
            },
            Direction::Up => {
                self.y += 1;
            },
            Direction::Down => {
                self.y -= 1;
            },
        }
    }

    fn follow(&mut self, other: &Pos) {
        match other.x - self.x {
            2 => {
                self.x += 1;
                match other.y - self.y {
                    2 | 1 => self.y += 1,
                    -2 | -1 => self.y -= 1,
                    _ => {}
                }
                // self.y = other.y;
            }
            -2 => {
                self.x -= 1;
                match other.y - self.y {
                    2 | 1 => self.y += 1,
                    -2 | -1 => self.y -= 1,
                    _ => {}
                }
                // self.y = other.y;
            }
            _ => {}
        }
        match other.y - self.y {
            2 => {
                self.y += 1;
                match other.x - self.x {
                    2 | 1 => self.x += 1,
                    -2 | -1 => self.x -= 1,
                    _ => {}
                }
                // self.x = other.x;
            }
            -2 => {
                self.y -= 1;
                match other.x - self.x {
                    2 | 1 => self.x += 1,
                    -2 | -1 => self.x -= 1,
                    _ => {}
                }
                // self.x = other.x;
            }
            _ => {}
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Snake {
    body: Vec<Pos>,
    size: usize,
}

impl Snake {
    fn new(length: usize) -> Snake {
        Snake{
            body: iter::repeat(Pos{x: 0, y: 0}).take(length).collect(),
            size: length,
        }
    }

    fn do_move(&mut self, dir: Direction) -> Pos {
        self.body[0].do_move(dir);

        for i in 1..self.size {
            let prev = self.body[i - 1].clone();
            self.body[i].follow(&prev);
        }

        // self.show(dir);
        self.body[self.size - 1].clone()
    }

    fn show(&self, dir: Direction) {
        let m = 11;
        let n = 2;

        let mut tiles: Vec<Vec<String>> = iter::repeat(
            iter::repeat(".".to_owned()).take(m).collect()
        ).take(m).collect();

        println!("{:?}: {}", dir, self.body[0]);
        for (i, p) in self.body.iter().enumerate() {
            let s = if i == 0 {
                "H".to_owned()
            } else {
                i.to_string()
            };
            tiles[(m - 1) - (p.y + n) as usize][(p.x + n) as usize] = s;
        }

        tiles.into_iter().for_each(|v| {
            println!("{}", String::from_iter(v))
        });
        println!("\n\n");
    }
}

fn solve(lines: Vec<String>, snake_len: usize) -> u32 {
    let mut s = Snake::new(snake_len);
    to_directions(lines)
        .into_iter()
        .flat_map(|(dir, n)| {
            iter::repeat(dir).take(n)
        })
        .map(|dir| s.do_move(dir) )
        .collect::<HashSet<Pos>>()
        .len() as u32
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    solve(lines, 2)
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    // 2370 => too low
    solve(lines, 10)
}

fn to_directions(lines: Vec<String>) -> Vec<(Direction, usize)> {
    lines.into_iter().map(|l| {
        let parts: Vec<String> = l.split(" ").map(|p| p.to_owned()).collect();
        let direction = match parts[0].as_str() {
            "R" => Some(Direction::Right),
            "L" => Some(Direction::Left),
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            _ => None
        }.unwrap();

        (direction, parts[1].parse::<usize>().unwrap())
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::day9::day9::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "R 4".to_owned(),
            "U 4".to_owned(),
            "L 3".to_owned(),
            "D 1".to_owned(),
            "R 4".to_owned(),
            "D 1".to_owned(),
            "L 5".to_owned(),
            "R 2".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 13)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "R 4".to_owned(),
            "U 4".to_owned(),
            "L 3".to_owned(),
            "D 1".to_owned(),
            "R 4".to_owned(),
            "D 1".to_owned(),
            "L 5".to_owned(),
            "R 2".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 1)
    }

    #[test]
    fn test_solve_p2_larger() {
        let lines: Vec<String> = vec![
            "R 5".to_owned(),
            "U 8".to_owned(),
            "L 8".to_owned(),
            "D 3".to_owned(),
            "R 17".to_owned(),
            "D 10".to_owned(),
            "L 25".to_owned(),
            "U 20".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 36)
    }
}

