use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Simulation {
    grid: HashMap<(i32, i32), Material>,
    low_x: i32,
    high_x: i32,
    low_y: i32,
    high_y: i32,
    floor: Option<i32>,

}

impl Simulation {
    fn new(with_floor: bool) -> Simulation {
        Simulation{
            grid: Default::default(),
            low_x: 99999,
            high_x: -99999,
            low_y: 99999,
            high_y: -99999,
            floor: if with_floor { Some(-99999) } else { None },
        }
    }

    fn set(&mut self, location: (i32, i32), material: Material) {
        self.grid.insert(location, material);
        if location.0 < self.low_x {
            self.low_x = location.0;
        }
        if location.0 > self.high_x {
            self.high_x = location.0;
        }
        if location.1 < self.low_y {
            self.low_y = location.1;
        }
        if location.1 > self.high_y {
            self.high_y = location.1;
        }
    }

    fn mark_rock_line(&mut self, start: (i32, i32), end: (i32, i32)) {
        let sx = if start.0 < end.0 { start.0 } else { end.0 };
        let ex = if start.0 > end.0 { start.0 } else { end.0 };
        let sy = if start.1 < end.1 { start.1 } else { end.1 };
        let ey = if start.1 > end.1 { start.1 } else { end.1 };

        for x in sx..ex + 1 {
            for y in sy..ey + 1 {
                self.set((x, y), Material::Rock);
                self.floor = self.floor.map(|_| self.high_y + 2);
            }
        }
    }

    fn at(&self, location: (i32, i32)) -> Material {
        match self.grid.get(&location) {
            None => {
                match self.floor {
                    None => Material::Air,
                    Some(y) => {
                        if location.1 == y {
                            Material::Rock
                        } else {
                            Material::Air
                        }
                    }
                }
            },
            Some(x) => *x,
        }
    }

    fn drop_sand(&mut self, drop_at: (i32, i32)) -> Option<(i32, i32)> {
        if self.floor.is_none() && drop_at.1 >= self.high_y {
            return None;
        }

        // look down
        match self.at((drop_at.0, drop_at.1 + 1)) {
            Material::Air => self.drop_sand((drop_at.0, drop_at.1 + 1)),
            _ => {
                // look down-left
                match self.at((drop_at.0 - 1, drop_at.1 + 1)) {
                    Material::Air => self.drop_sand((drop_at.0 - 1, drop_at.1 + 1)),
                    _ => {
                        // look down-right
                        match self.at((drop_at.0 + 1, drop_at.1 + 1)) {
                            Material::Air => self.drop_sand((drop_at.0 + 1, drop_at.1 + 1)),
                            _ => {
                                // stop falling
                                self.set(drop_at, Material::Sand);
                                Some(drop_at)
                            }
                        }
                    }
                }
            }
        }
    }

    fn show(&self) {
        let y_lim = match self.floor {
            Some(y) => y + 1,
            None => self.high_y + 1
        };
        for y in self.low_y..y_lim {
            let mut line: String = "".to_owned();
            for x in self.low_x..self.high_x + 1 {
                line.push(match self.at((x, y)) {
                    Material::Air => '.',
                    Material::Rock => '#',
                    Material::Sand => 'o'
                });
            }
            println!("{}", line);
        }
    }
}

fn draw_rocks(lines: Vec<String>, sim: &mut Simulation) {
    lines.iter()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let parts = coords.split(",").collect::<Vec<&str>>();
                    (
                        parts[0].parse::<i32>().unwrap(),
                        parts[1].parse::<i32>().unwrap(),
                    )
                }).collect::<Vec<(i32, i32)>>()
                .windows(2)
                .map(|parts| (parts[0], parts[1]))
                .collect::<Vec<((i32, i32), (i32, i32))>>()
        })
        .for_each(|(from, to)| {
            sim.mark_rock_line(from, to);
        });
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    let mut sim = Simulation::new(false);
    draw_rocks(lines, &mut sim);

    let mut count = 0;
    loop {
        match sim.drop_sand((500, 0)) {
            None => return count,
            _ => count += 1,
        }
    }
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    let mut sim = Simulation::new(true);
    draw_rocks(lines, &mut sim);

    let source = (500, 0);
    let mut count = 0;
    loop {
        count += 1;
        match sim.drop_sand(source) {
            None => panic!("should never happen with a floor"),
            Some(loc) => {
                if loc == source {
                    return count;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::day14::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "498,4 -> 498,6 -> 496,6".to_owned(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 24)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "498,4 -> 498,6 -> 496,6".to_owned(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 93)
    }
}

