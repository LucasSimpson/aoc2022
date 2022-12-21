use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div
}

impl Op {
    fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Const(Option<isize>),
    Math(String, Op, String),
}

fn resolve(cache: &mut HashMap<String, Option<isize>>, key: String, map: &HashMap<String, Operation>) -> Option<isize> {
    if let Some(res) = cache.get(&key) {
        return res.clone()
    }

    let res = {
        match map.get(&key).unwrap() {
            Operation::Const(res) => res.clone(),
            Operation::Math(left, op, right) => {
                let res_left = resolve(cache, left.clone(), map);
                let res_right = resolve(cache, right.clone(), map);
                res_left.and_then(|left_res| {
                    res_right.map(|right_res| {
                        op.apply(left_res, right_res)
                    })
                })
            },
        }
    };

    cache.insert(key, res);
    res
}

fn fill_blank(cache: &HashMap<String, Option<isize>>, key: String, map: &HashMap<String, Operation>, target: isize) -> isize {
    match map.get(&key).unwrap() {
        Operation::Const(None) => target,
        Operation::Math(left, op, right) => {
            match cache.get(left).unwrap() {
                Some(left) => {
                    assert!(cache.get(right).unwrap().is_none());
                    let new_target = match op {
                        Op::Add => target - left, // target = left + X
                        Op::Sub => left - target, // target = left - X
                        Op::Mul => target / left, // target = left * X
                        Op::Div => left / target, // target = left / X
                    };
                    fill_blank(cache, right.clone(), map, new_target)
                }
                None => {
                    let right = cache.get(right).unwrap().unwrap();
                    let new_target = match op {
                        Op::Add => target - right, // target = X + right
                        Op::Sub => target + right, // target = X - right
                        Op::Mul => target / right, // target = X * right
                        Op::Div => target * right, // target = X / right
                    };
                    fill_blank(cache, left.clone(), map, new_target)
                }
            }
        },
        _ => panic!("should never happen"),
    }
}

fn parse_line(line: String) -> (String, Operation) {
    let re1 = Regex::new(r"([a-z]+): ([a-z]+) ([+\-*/]) ([a-z]+)").unwrap();
    let re2 = Regex::new(r"([a-z]+): (-?\d+)").unwrap();

    if let Some(captures) = re1.captures(&line) {
        let id = captures[1].to_owned();
        let left = captures[2].to_owned();
        let op = captures[3].parse::<Op>().unwrap();
        let right = captures[4].to_owned();
        (id, Operation::Math(left, op, right))
    } else {
        let captures = re2.captures(&line).unwrap();
        let id = captures[1].to_owned();
        (id, Operation::Const(captures[2].parse::<isize>().ok()))
    }
}

pub fn solve_p1(lines: Vec<String>) -> u64 {
    let map: HashMap<String, Operation> = lines.into_iter().map(parse_line).collect();
    let mut cache: HashMap<String, Option<isize>> = HashMap::new();
    resolve(&mut cache, "root".to_owned(), &map).unwrap_or(0) as u64
}

pub fn solve_p2(lines: Vec<String>) -> u64 {
    let mut map: HashMap<String, Operation> = lines.into_iter().map(parse_line).collect();
    map.insert("humn".to_owned(), Operation::Const(None)); // set humn (me) to None

    let mut cache: HashMap<String, Option<isize>> = HashMap::new();
    resolve(&mut cache, "root".to_owned(), &map); // solve what we can

    match map.get("root").unwrap() {
        Operation::Const(r) => panic!("dafuq"),
        Operation::Math(left, _, right) => {
            let (target, key) = match cache.get(left).unwrap() {
                Some(val) => (*val, right),
                None => {
                    let val = cache.get(right).unwrap().unwrap();
                    (val, left)
                },
            };
            fill_blank(&cache, key.clone(), &map, target) as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day21::day21::{Operation, parse_line, resolve, solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "root: pppw + sjmn".to_owned(),
            "dbpl: 5".to_owned(),
            "cczh: sllz + lgvd".to_owned(),
            "zczc: 2".to_owned(),
            "ptdq: humn - dvpt".to_owned(),
            "dvpt: 3".to_owned(),
            "lfqf: 4".to_owned(),
            "humn: 5".to_owned(),
            "ljgn: 2".to_owned(),
            "sjmn: drzm * dbpl".to_owned(),
            "sllz: 4".to_owned(),
            "pppw: cczh / lfqf".to_owned(),
            "lgvd: ljgn * ptdq".to_owned(),
            "drzm: hmdt - zczc".to_owned(),
            "hmdt: 32".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 152)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "root: pppw + sjmn".to_owned(),
            "dbpl: 5".to_owned(),
            "cczh: sllz + lgvd".to_owned(),
            "zczc: 2".to_owned(),
            "ptdq: humn - dvpt".to_owned(),
            "dvpt: 3".to_owned(),
            "lfqf: 4".to_owned(),
            "humn: 5".to_owned(),
            "ljgn: 2".to_owned(),
            "sjmn: drzm * dbpl".to_owned(),
            "sllz: 4".to_owned(),
            "pppw: cczh / lfqf".to_owned(),
            "lgvd: ljgn * ptdq".to_owned(),
            "drzm: hmdt - zczc".to_owned(),
            "hmdt: 32".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 301)
    }
}

