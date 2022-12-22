use std::collections::HashMap;
use std::str::{Chars, FromStr};

#[derive(Debug, Clone)]
enum Tile {
    Open,
    Rock,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Open),
            "#" => Ok(Tile::Rock),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn turn(&self, td: &TurnDirection) -> Facing {
        match td {
            TurnDirection::Right => {
                match self {
                    Facing::Up => Facing::Right,
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                }
            },
            TurnDirection::Left => {
                match self {
                    Facing::Up => Facing::Left,
                    Facing::Left => Facing::Down,
                    Facing::Down => Facing::Right,
                    Facing::Right => Facing::Up,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(usize),
    Turn(TurnDirection),
}

#[derive(Debug, Clone)]
struct Map {
    tiles: HashMap<(usize, usize), Tile>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn from_lines(lines: Vec<String>) -> Map {
        let tiles: Vec<((usize, usize), Tile)> = lines.iter()
            .take_while(|l| l.len() > 0)
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| c.to_string().parse::<Tile>().ok().map(|t| ((x + 1, y + 1), t)))
                    .collect::<Vec<((usize, usize), Tile)>>()
            })
            .collect();

        let mx = tiles.iter().map(|((x, _), _)| x).max().unwrap().clone() + 1;
        let my = tiles.iter().map(|((_, y), _)| y).max().unwrap().clone() + 1;
        Map{
            tiles: tiles.into_iter().collect(),
            max_x: mx,
            max_y: my,
        }
    }

    fn start_pos(&self) -> ((usize, usize), Facing) {
        let mut x = 1;
        loop {
            if let Some(Tile::Open) = self.tiles.get(&(x, 1)) {
                return ((x, 1), Facing::Right)
            }
            x += 1;
        }
    }

    fn move_once(pos: (usize, usize), facing: &Facing) -> (usize, usize) {
        match facing {
            Facing::Up => (pos.0, pos.1 - 1),
            Facing::Down => (pos.0, pos.1 + 1),
            Facing::Right => (pos.0 + 1, pos.1),
            Facing::Left => (pos.0 - 1, pos.1),
        }
    }

    fn next_tile(&self, pos: (usize, usize), facing: &Facing) -> ((usize, usize), Tile) {
        let mut move_to = Map::move_once(pos, facing);
        match self.tiles.get(&move_to) {
            None => {
                // wrap around
                move_to = match facing {
                    Facing::Up => (pos.0, self.max_y - 1),
                    Facing::Down => (pos.0, 1),
                    Facing::Right => (1, pos.1),
                    Facing::Left => (self.max_x - 1, pos.1),
                };
                let mut i = 0;
                loop {
                    i += 1;
                    if i > 1000 {
                        panic!("error");
                    }
                    match self.tiles.get(&move_to) {
                        None => {},
                        Some(tile) => return (move_to, tile.clone()),
                    }
                    move_to = Map::move_once(move_to, facing);
                };
            },
            Some(tile) => (move_to, tile.clone()),
        }
    }

    fn do_move(&self, starting_pos: (usize, usize), facing: &Facing, n: usize) -> (usize, usize) {
        let mut pos = starting_pos;
        for _ in 0..n {
            match self.next_tile(pos, facing) {
                (new_pos, Tile::Open) => pos = new_pos,
                (_, Tile::Rock) => return pos,
            }
        }
        pos
    }

    fn show(&self, trace: &HashMap<(usize, usize), Facing>) {
        for y in 1..self.max_y {
            let mut line = "".to_owned();
            for x in 1..self.max_x {
                line.push({
                    match trace.get(&(x, y)) {
                        None => {
                            match self.tiles.get(&(x, y)) {
                                None => ' ',
                                Some(Tile::Open) => '.',
                                Some(Tile::Rock) => '#',
                            }
                        },
                        Some(facing) => {
                            match facing {
                                Facing::Up => '^',
                                Facing::Down => 'v',
                                Facing::Left => '<',
                                Facing::Right => '>',
                            }
                        }
                    }
                })
            }
            println!("{}", line);
        }
    }
}

fn parse_instructions(current: Option<char>, iter: &mut Chars) -> Vec<Instruction> {
    let mut res = Vec::new();

    match current {
        Some('R') => {
            res.push(Instruction::Turn(TurnDirection::Right));
            res.append(&mut parse_instructions(iter.next(), iter));
        },
        Some('L') => {
            res.push(Instruction::Turn(TurnDirection::Left));
            res.append(&mut parse_instructions(iter.next(), iter));
        },
        Some(x) => {
            let mut buf = x.to_string();
            loop {
                match iter.next() {
                    None => {
                        res.push(Instruction::Move(buf.parse::<usize>().unwrap()));
                        break;
                    },
                    Some(c) => {
                        if c == 'R' || c == 'L' {
                            res.push(Instruction::Move(buf.parse::<usize>().unwrap()));
                            res.append(&mut parse_instructions(Some(c), iter));
                            break;
                        } else {
                            buf.push(c);
                        }
                    }
                }
            }
        },
        None => {},
    };

    res
}

pub fn solve_p1(lines: Vec<String>) -> u64 {
    let mut iter = lines.last().unwrap().chars();
    let instructions = parse_instructions(iter.next(), &mut iter);
    let map = Map::from_lines(lines);

    let ((column, row), facing) = instructions.iter()
        .fold(map.start_pos(), |(pos, facing), instruction| {
            match instruction {
                Instruction::Turn(td) => (pos, facing.turn(td)),
                Instruction::Move(x) => (map.do_move(pos, &facing, *x), facing),
            }
        });

    let facing_coef = match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };

    (1000 * row + 4 * column + facing_coef) as u64
}

#[derive(Debug)]
struct Zip {
    new_face: usize,
    map_pos: fn((usize, usize)) -> ((usize, usize)),
    new_facing: Facing,
}

#[derive(Debug)]
struct CubeFace {
    id: usize,
    size: usize,
    tiles: Vec<Vec<Tile>>,
    global_x: usize,
    global_y: usize,

    top: Zip,
    left: Zip,
    right: Zip,
    bottom: Zip,
}

impl CubeFace {
    fn do_move(&self, pos: (usize, usize), facing: Facing) -> (usize, (usize, usize), Facing) {
        let new_pos = match facing {
            Facing::Up => (pos.0, pos.1 - 1),
            Facing::Down => (pos.0, pos.1 + 1),
            Facing::Right => (pos.0 + 1, pos.1),
            Facing::Left => (pos.0 - 1, pos.1),
        };
        if new_pos.0 == 0 {
            (self.left.new_face, self.left.map_pos(new_pos), self.left.new_facing.clone())
        }

        (0, (0, 0), Facing::Down)
    }

}

#[derive(Debug)]
struct Cube {
    faces: Vec<Cube>
}


pub fn solve_p2(lines: Vec<String>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day22::day22::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "        ...#".to_owned(),
            "        .#..".to_owned(),
            "        #...".to_owned(),
            "        ....".to_owned(),
            "...#.......#".to_owned(),
            "........#...".to_owned(),
            "..#....#....".to_owned(),
            "..........#.".to_owned(),
            "        ...#....".to_owned(),
            "        .....#..".to_owned(),
            "        .#......".to_owned(),
            "        ......#.".to_owned(),
            "".to_owned(),
            "10R5L5R10L4R5L5".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 6032)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "        ...#".to_owned(),
            "        .#..".to_owned(),
            "        #...".to_owned(),
            "        ....".to_owned(),
            "...#.......#".to_owned(),
            "........#...".to_owned(),
            "..#....#....".to_owned(),
            "..........#.".to_owned(),
            "        ...#....".to_owned(),
            "        .....#..".to_owned(),
            "        .#......".to_owned(),
            "        ......#.".to_owned(),
            "".to_owned(),
            "10R5L5R10L4R5L5".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 5031)
    }
}

