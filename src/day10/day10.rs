use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(" ")
            .map(|s| s.to_owned())
            .collect();

        match parts[0].as_str() {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(parts[1].parse::<i64>().unwrap())),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct CPU {
    x: i64,
    cycle: usize,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1,
            cycle: 1,
        }
    }

    fn process(&mut self, instruction: Instruction) -> (i64, char) {
        let c64 = self.cycle as i64;
        let ss = self.x * (c64);
        let output: char = if self.x + 1 >= (c64 % 40) - 1 && self.x + 1 <= (c64 % 40) + 1 {
            '#'
        } else {
            '.'
        };

        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
            },
            Instruction::Addx(opp) => {
                self.cycle += 1;
                self.x += opp;
            }
        };

        (ss, output)
    }
}

fn sneaky_transform(instructions: Vec<Instruction>) -> Vec<Instruction> {
    instructions.into_iter().flat_map(|i| {
        match i {
            Instruction::Noop => vec![i],
            Instruction::Addx(opp) => vec![Instruction::Noop, Instruction::Addx(opp)]
        }
    }).collect()
}

pub fn solve_p1(lines: Vec<String>) -> i64 {
    let mut cpu = CPU::new();

    let instructions = lines.into_iter()
        .map(|l| Instruction::from_str(l.as_str()).unwrap() )
        .collect();

    sneaky_transform(instructions).into_iter()
        .map(|i| cpu.process(i) )
        .map(|(ss, _)| ss )
        .skip(19)
        .step_by(40)
        .sum()
}

pub fn solve_p2(lines: Vec<String>) -> i64 {
    let mut cpu = CPU::new();

    let instructions = lines.into_iter()
        .map(|l| Instruction::from_str(l.as_str()).unwrap() )
        .collect();

    sneaky_transform(instructions).into_iter()
        .map(|i| cpu.process(i))
        .map(|(_, output)| output )
        .collect::<Vec<char>>().chunks(40)
        .map(|line| String::from_iter(line) )
        .for_each(|line| println!("{}", line));

    13140
}

#[cfg(test)]
mod tests {
    use crate::day10::day10::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "noop".to_owned(),
            "addx 3".to_owned(),
            "addx -5".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 0)
    }

    #[test]
    fn test_solve_p1_large() {
        let lines: Vec<String> = vec![
            "addx 15".to_owned(),
            "addx -11".to_owned(),
            "addx 6".to_owned(),
            "addx -3".to_owned(),
            "addx 5".to_owned(),
            "addx -1".to_owned(),
            "addx -8".to_owned(),
            "addx 13".to_owned(),
            "addx 4".to_owned(),
            "noop".to_owned(),
            "addx -1".to_owned(),
            "addx 5".to_owned(),
            "addx -1".to_owned(),
            "addx 5".to_owned(),
            "addx -1".to_owned(),
            "addx 5".to_owned(),
            "addx -1".to_owned(),
            "addx 5".to_owned(),
            "addx -1".to_owned(),
            "addx -35".to_owned(),
            "addx 1".to_owned(),
            "addx 24".to_owned(),
            "addx -19".to_owned(),
            "addx 1".to_owned(),
            "addx 16".to_owned(),
            "addx -11".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 21".to_owned(),
            "addx -15".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx -3".to_owned(),
            "addx 9".to_owned(),
            "addx 1".to_owned(),
            "addx -3".to_owned(),
            "addx 8".to_owned(),
            "addx 1".to_owned(),
            "addx 5".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx -36".to_owned(),
            "noop".to_owned(),
            "addx 1".to_owned(),
            "addx 7".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 2".to_owned(),
            "addx 6".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 7".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "addx -13".to_owned(),
            "addx 13".to_owned(),
            "addx 7".to_owned(),
            "noop".to_owned(),
            "addx 1".to_owned(),
            "addx -33".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 2".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 8".to_owned(),
            "noop".to_owned(),
            "addx -1".to_owned(),
            "addx 2".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "addx 17".to_owned(),
            "addx -9".to_owned(),
            "addx 1".to_owned(),
            "addx 1".to_owned(),
            "addx -3".to_owned(),
            "addx 11".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx -13".to_owned(),
            "addx -19".to_owned(),
            "addx 1".to_owned(),
            "addx 3".to_owned(),
            "addx 26".to_owned(),
            "addx -30".to_owned(),
            "addx 12".to_owned(),
            "addx -1".to_owned(),
            "addx 3".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx -9".to_owned(),
            "addx 18".to_owned(),
            "addx 1".to_owned(),
            "addx 2".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 9".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx -1".to_owned(),
            "addx 2".to_owned(),
            "addx -37".to_owned(),
            "addx 1".to_owned(),
            "addx 3".to_owned(),
            "noop".to_owned(),
            "addx 15".to_owned(),
            "addx -21".to_owned(),
            "addx 22".to_owned(),
            "addx -6".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "addx 2".to_owned(),
            "addx 1".to_owned(),
            "noop".to_owned(),
            "addx -10".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "addx 20".to_owned(),
            "addx 1".to_owned(),
            "addx 2".to_owned(),
            "addx 2".to_owned(),
            "addx -6".to_owned(),
            "addx -11".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
            "noop".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 13140)
    }

    // #[test]
    // fn test_solve_p2() {
    //     let lines: Vec<String> = vec![
    //     ];
    //     assert_eq!(solve_p2(lines), 70)
    // }
}

