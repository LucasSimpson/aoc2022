use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::{Chars, FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Empty,
    Arr(Vec<Packet>),
    Val(usize),
}

impl Packet {
    fn consume(iter: &mut Chars) -> Result<(Self, Option<char>), ()> {
        let first = iter.next().unwrap();
        match first {
            '[' => {
                let mut elems: Vec<Packet> = Vec::new();
                loop {
                    let (packet, current_char) = Packet::consume(iter)?;
                    match packet {
                        Packet::Empty => {},
                        _ => elems.push(packet),
                    }
                    match current_char {
                        Some(',') => {
                            continue
                        },
                        Some(']') => {
                            return Ok((Packet::Arr(elems), iter.next()))
                        },
                        None => {
                            return Ok((Packet::Arr(elems), None))
                        }
                        _ => return Err(()),
                    }
                }
            },
            ']' => {
                return Ok((Packet::Empty, Some(first)))
            }
            _ => {
                let mut val_string = "".to_owned();
                val_string.push(first);

                loop {
                    let next = iter.next().unwrap();
                    if !next.is_ascii_digit() {
                        return val_string.parse::<usize>()
                            .map(|x| (Packet::Val(x), Some(next)))
                            .map_err(|_| ());
                    }
                    val_string.push(next);
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Packet::Empty => "EMPTY".to_owned(),
            Packet::Val(x) => format!("{}", x),
            Packet::Arr(x) => {
                let mut res = "[".to_owned();
                res.push_str(&x.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(","));
                res.push(']');
                res
            },
        }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Packet::consume(&mut s.chars()).map(|(p, _)| p)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Empty, Packet::Empty) => {
                Ordering::Equal
            },
            (Packet::Empty, _) => {
                Ordering::Less
            },
            (Packet::Val(_), Packet::Empty) => {
                Ordering::Greater
            },
            (Packet::Val(x), Packet::Val(y)) => {
                x.cmp(y)
            },
            (Packet::Val(x), Packet::Arr(y)) => {
                Vec::from([Packet::Val(*x)]).cmp(y)
            },
            (Packet::Arr(_), Packet::Empty) => {
                Ordering::Greater
            },
            (Packet::Arr(x), Packet::Val(y)) => {
                x.cmp(&Vec::from([Packet::Val(*y)]))
            },
            (Packet::Arr(x), Packet::Arr(y)) => {
                x.cmp(y)
            },
        }
    }
}



pub fn solve_p1(lines: Vec<String>) -> u32 {
    lines.chunks(3)
        .map(|c | (c[0].clone(), c[1].clone()))
        .map(|(left, right)| {
            (
                Packet::from_str(&left).unwrap(),
                Packet::from_str(&right).unwrap(),
            )
        })
        .enumerate()
        .map(|(index, (left, right))| {
            if left < right  {
                index as u32 + 1
            } else {
                0
            }
        })
        .sum()
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    let first_div_packet = Packet::Arr(vec![Packet::Arr(vec![Packet::Val(2)])]);
    let second_div_packet = Packet::Arr(vec![Packet::Arr(vec![Packet::Val(6)])]);

    let mut packets: Vec<Packet> = lines.chunks(3)
        .map(|c | (c[0].clone(), c[1].clone()))
        .flat_map(|(left, right)| {
            Vec::from([
                Packet::from_str(&left).unwrap(),
                Packet::from_str(&right).unwrap(),
            ])
        })
        .collect();

    packets.push(first_div_packet.clone());
    packets.push(second_div_packet.clone());
    packets.sort();

    let mut index1 = 0;
    let mut index2 = 0;
    for (i, p) in packets.iter().enumerate() {
        if *p == first_div_packet {
            index1 = i + 1;
        } else if *p == second_div_packet {
            index2 = i + 1;
        }
    }
    (index1 * index2) as u32
}

#[cfg(test)]
mod tests {
    use crate::day13::day13::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1_edge() {
        let lines: Vec<String> = vec![
            "[]".to_owned(),
            "[[],6]".to_owned(),
            "".to_owned(),
            "[[],6]".to_owned(),
            "[[[],6]]".to_owned(),
            "".to_owned(),
            "[[[6,[5,2,3,6,8]],[[9,7],[6,10,8],4,3,[3,1,9,4]]],[3,[[7,8]]],[7,0,[6],1]]".to_owned(),
            "[[[],6,7,[4,1],[6,[],[4,7,0,0,6],[9,2,7,9],0]]]".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 3)
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "[1,1,3,1,1]".to_owned(),
            "[1,1,5,1,1]".to_owned(),
            "".to_owned(),
            "[[1],[2,3,4]]".to_owned(),
            "[[1],4]".to_owned(),
            "".to_owned(),
            "[9]".to_owned(),
            "[[8,7,6]]".to_owned(),
            "".to_owned(),
            "[[4,4],4,4]".to_owned(),
            "[[4,4],4,4,4]".to_owned(),
            "".to_owned(),
            "[7,7,7,7]".to_owned(),
            "[7,7,7]".to_owned(),
            "".to_owned(),
            "[]".to_owned(),
            "[3]".to_owned(),
            "".to_owned(),
            "[[[]]]".to_owned(),
            "[[]]".to_owned(),
            "".to_owned(),
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_owned(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 13)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "[1,1,3,1,1]".to_owned(),
            "[1,1,5,1,1]".to_owned(),
            "".to_owned(),
            "[[1],[2,3,4]]".to_owned(),
            "[[1],4]".to_owned(),
            "".to_owned(),
            "[9]".to_owned(),
            "[[8,7,6]]".to_owned(),
            "".to_owned(),
            "[[4,4],4,4]".to_owned(),
            "[[4,4],4,4,4]".to_owned(),
            "".to_owned(),
            "[7,7,7,7]".to_owned(),
            "[7,7,7]".to_owned(),
            "".to_owned(),
            "[]".to_owned(),
            "[3]".to_owned(),
            "".to_owned(),
            "[[[]]]".to_owned(),
            "[[]]".to_owned(),
            "".to_owned(),
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_owned(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 140)
    }
}

