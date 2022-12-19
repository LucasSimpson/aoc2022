use std::fmt::{Display, Formatter};
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone)]
struct Ring {
    size: u32,
    value: u32,
}

impl Ring {
    fn add_const(&mut self, value: u32) {
        self.value = (self.value + value) % self.size;
    }

    fn add_ring(&mut self, other: &Ring) {
        if other.size != self.size {
            panic!("cannot add rings of different sizes");
        }
        self.add_const(other.value);
    }

    fn mul_const(&mut self, value: u32) {
        self.value = (self.value * value) % self.size;
    }

    fn mul_ring(&mut self, other: &Ring) {
        if other.size != self.size {
            panic!("cannot add rings of different sizes");
        }
        self.mul_const(other.value);
    }
}

#[derive(Debug, Clone)]
struct DivisorChecker {
    rings: Vec<Ring>,
}

impl DivisorChecker {
    fn new(initial: u32) -> DivisorChecker {
        DivisorChecker {
            rings: (1..30).map(|x| Ring{
                size: x,
                value: initial % x,
            }).collect(),
        }
    }

    fn divisible_by(&self, value: u32) -> bool {
        if value >= 30 {
            panic!("cannot check divisible by for value >= 20");
        }


        self.rings[value as usize - 1].value == 0
    }

    fn add_const(mut self, value: u32) -> Self {
        self.rings.iter_mut().for_each(|r| r.add_const(value));
        self
    }

    fn add_divisor_checker(mut self, dc: &DivisorChecker) -> Self {
        self.rings.iter_mut().enumerate().for_each(|(i, r)| r.add_ring(&dc.rings[i]));
        self
    }

    fn mul_const(mut self, value: u32) -> Self {
        self.rings.iter_mut().for_each(|r| r.mul_const(value));
        self
    }

    fn mul_divisor_checker(mut self, dc: &DivisorChecker) -> Self {
        self.rings.iter_mut().enumerate().for_each(|(i, r)| r.mul_ring(&dc.rings[i]));
        self
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Value(u32)
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>().map(|x| Operand::Value(x)).or_else(|_| Ok(Operand::Old))
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => Err(()),
        }
    }
}

impl Operation {
    fn apply(&self, old: &DivisorChecker, left: &Operand, right: &Operand) -> DivisorChecker {
        match self {
            Operation::Add => Operation::add(old, left, right),
            Operation::Mul => Operation::mul(old, left, right),
        }
    }

    fn add(old: &DivisorChecker, left: &Operand, right: &Operand) -> DivisorChecker {
        match left {
            Operand::Old => {
                match right {
                    Operand::Old => {
                        old.clone().add_divisor_checker(&old)
                    },
                    Operand::Value(value) => {
                        old.clone().add_const(value.to_owned())
                    }
                }
            }
            Operand::Value(value) => {
                match right {
                    Operand::Old => {
                        old.clone().add_const(value.to_owned())
                    },
                    Operand::Value(right) => DivisorChecker::new(value * right)
                }
            }
        }
    }

    fn mul(old: &DivisorChecker, left: &Operand, right: &Operand) -> DivisorChecker {
        match left {
            Operand::Old => {
                match right {
                    Operand::Old => {
                        old.clone().mul_divisor_checker(&old)
                    },
                    Operand::Value(value) => {
                        old.clone().mul_const(value.to_owned())
                    }
                }
            }
            Operand::Value(value) => {
                match right {
                    Operand::Old => {
                        old.clone().mul_const(value.to_owned())
                    },
                    Operand::Value(right) => DivisorChecker::new(value * right)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<DivisorChecker>,
    operation: Operation,
    left: Operand,
    right: Operand,
    divisor: u32,
    throw_true: usize,
    throw_false: usize,
    inspected: usize,
    worry_factor: u32,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "items={:?}, left={:?}, operation={:?}, right={:?}, div={:?}, throw_true={}, throw_false={}",
            "self.items",
            self.left,
            self.operation,
            self.right,
            self.divisor,
            self.throw_true,
            self.throw_false,
        )
    }
}

impl Monkey {
    fn do_round(&mut self) -> Vec<(usize, DivisorChecker)> {
        self.items.drain(..).map(|worry| {
            self.inspected += 1;
            let new_worry = self.operation.apply(&worry, &self.left, &self.right);
            let throw_to = if new_worry.divisible_by(self.divisor) {
                self.throw_true
            } else {
                self.throw_false
            };
            (throw_to, new_worry)
        }).collect()
    }

    fn push(&mut self, worry: DivisorChecker) {
        self.items.push(worry);
    }
}

fn show_monkeys(i: usize, monkeys: &Vec<Monkey>) {
    println!("== After round {} ==", i + 1);
    for (i, m) in monkeys.iter().enumerate() {
        println!("\tMonkey {} inspected {} times: items={:?}", i, m.inspected, "m.items");
    }
}


fn simulate(lines: Vec<String>, worry_factor: u32, rounds: usize) -> u64 {
    let re = Regex::new(r"Monkey (?P<id>\d+):\s*Starting items:\s*(?P<items>(\d+(, )?)+)\s*Operation:\s*new = (?P<left>\d+|old) (?P<operation>\+|\*|\\|-) (?P<right>\d+|old)\s*Test: divisible by (?P<div>\d+)\s*If true: throw to monkey (?P<id_true>\d+)\s*If false: throw to monkey (?P<id_false>\d+)").unwrap();

    let mut monkeys = lines.chunks(7)
        .map(|lines| {
            let s = String::from_iter(lines.into_iter().map(|s| s.to_owned()));
            let matches = re.captures(&s).unwrap();
            Monkey{
                items: matches.name("items")
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .map(DivisorChecker::new)
                    .collect::<Vec<DivisorChecker>>(),
                operation: matches.name("operation").unwrap().as_str().parse::<Operation>().unwrap(),
                left: matches.name("left").unwrap().as_str().parse::<Operand>().unwrap(),
                right: matches.name("right").unwrap().as_str().parse::<Operand>().unwrap(),
                divisor: matches.name("div").unwrap().as_str().parse::<u32>().unwrap(),
                throw_true: matches.name("id_true").unwrap().as_str().parse::<usize>().unwrap(),
                throw_false: matches.name("id_false").unwrap().as_str().parse::<usize>().unwrap(),
                inspected: 0,
                worry_factor: worry_factor.clone(),
            }
        })
        .collect::<Vec<Monkey>>();

    for j in 0..rounds {
        for i in 0..monkeys.len() {
            let results =  monkeys[i].do_round();
            results.into_iter().for_each(|(throw_to, worry)| {
                monkeys[throw_to].push(worry);
            });
        }

        if j == 0 || j == 19 || j == 999 || j == 1999 || j == 2999 || j == 3999 || j == 4999 || j == 5999
            || j == 6999 || j == 7999 || j == 8999 || j == 9999 {
            show_monkeys(j, &monkeys);
        }
    }

    let mut sorted = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    sorted.sort();
    sorted.into_iter().rev().take(2).reduce(|x, y| x * y).unwrap() as u64
}

pub fn solve_p1(lines: Vec<String>) -> u64 {
    simulate(lines, 3, 20)
}

pub fn solve_p2(lines: Vec<String>) -> u64 {
    simulate(lines, 1, 10000 )
}

#[cfg(test)]
mod tests {
    use crate::day11::day11::{DivisorChecker, solve_p1, solve_p2};

    #[test]
    fn test_rings() {
        for x in 1..30 {
            for y in 1..100 {
                for j in 1..200 {
                    assert_eq!((y + j) % x == 0, DivisorChecker::new(y).add_const(j).divisible_by(x));
                    assert_eq!((y * j) % x == 0, DivisorChecker::new(y).mul_const(j).divisible_by(x));
                    assert_eq!((y + j) % x == 0, DivisorChecker::new(y).add_divisor_checker(&DivisorChecker::new(j)).divisible_by(x));
                    assert_eq!((y * j) % x == 0, DivisorChecker::new(y).mul_divisor_checker(&DivisorChecker::new(j)).divisible_by(x));
                }
            }
        }
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "Monkey 0:".to_owned(),
            "  Starting items: 79, 98".to_owned(),
            "  Operation: new = old * 19".to_owned(),
            "  Test: divisible by 23".to_owned(),
            "    If true: throw to monkey 2".to_owned(),
            "    If false: throw to monkey 3".to_owned(),
            "".to_owned(),
            "Monkey 1:".to_owned(),
            "  Starting items: 54, 65, 75, 74".to_owned(),
            "  Operation: new = old + 6".to_owned(),
            "  Test: divisible by 19".to_owned(),
            "    If true: throw to monkey 2".to_owned(),
            "    If false: throw to monkey 0".to_owned(),
            "".to_owned(),
            "Monkey 2:".to_owned(),
            "  Starting items: 79, 60, 97".to_owned(),
            "  Operation: new = old * old".to_owned(),
            "  Test: divisible by 13".to_owned(),
            "    If true: throw to monkey 1".to_owned(),
            "    If false: throw to monkey 3".to_owned(),
            "".to_owned(),
            "Monkey 3:".to_owned(),
            "  Starting items: 74".to_owned(),
            "  Operation: new = old + 3".to_owned(),
            "  Test: divisible by 17".to_owned(),
            "    If true: throw to monkey 0".to_owned(),
            "    If false: throw to monkey 1".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 10605)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "Monkey 0:".to_owned(),
            "  Starting items: 79, 98".to_owned(),
            "  Operation: new = old * 19".to_owned(),
            "  Test: divisible by 23".to_owned(),
            "    If true: throw to monkey 2".to_owned(),
            "    If false: throw to monkey 3".to_owned(),
            "".to_owned(),
            "Monkey 1:".to_owned(),
            "  Starting items: 54, 65, 75, 74".to_owned(),
            "  Operation: new = old + 6".to_owned(),
            "  Test: divisible by 19".to_owned(),
            "    If true: throw to monkey 2".to_owned(),
            "    If false: throw to monkey 0".to_owned(),
            "".to_owned(),
            "Monkey 2:".to_owned(),
            "  Starting items: 79, 60, 97".to_owned(),
            "  Operation: new = old * old".to_owned(),
            "  Test: divisible by 13".to_owned(),
            "    If true: throw to monkey 1".to_owned(),
            "    If false: throw to monkey 3".to_owned(),
            "".to_owned(),
            "Monkey 3:".to_owned(),
            "  Starting items: 74".to_owned(),
            "  Operation: new = old + 3".to_owned(),
            "  Test: divisible by 17".to_owned(),
            "    If true: throw to monkey 0".to_owned(),
            "    If false: throw to monkey 1".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 2713310158)
    }
}

