use std::collections::HashMap;
use regex::Regex;

fn contains(opened: u64, key: u64) -> bool {
    opened & key > 0
}

fn set(opened: u64, key: u64) -> u64 {
    opened | key
}

fn parse_lines(lines: Vec<String>) -> (HashMap<String, u64>, HashMap::<u64, (usize, Vec<u64>)>) {
    let re = Regex::new(r"Valve (?P<key>[A-Z]+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<leads_to>([A-Z]+(, )?)+)").unwrap();
    let keys_str = lines.into_iter()
        .map(|line| {
            let captures = re.captures(&line).unwrap();

            let key = (&captures[1]).to_owned();
            let rate = (&captures[2]).parse::<usize>().unwrap();
            let leads_to: Vec<String> = (&captures[3]).split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            (key, (rate, leads_to))

            // valves.insert(key, (rate, leads_to));
        })
        .collect::<Vec<(String, (usize, Vec<String>))>>();

    let keys_map: HashMap<String, u64> = keys_str.iter()
        .map(|(key, _)| key)
        .enumerate()
        .map(|(i, key)| {
            (key.clone(), 1 << i)
        })
        .collect();

    let mut valves = HashMap::<u64, (usize, Vec<u64>)>::new();
    keys_str.iter()
        .for_each(|(key, (rate, leads_to))| {
            valves.insert(
                keys_map.get(key).unwrap().to_owned(),
                (
                    rate.to_owned(),
                    leads_to.iter().map(|k| keys_map.get(k).unwrap()).cloned().collect(),
                )
            );
        });

    (keys_map, valves)
}

fn recurse_p1(
    cache: &mut HashMap<(usize, u64, u64), usize>,
    opened: u64,
    valves: &HashMap::<u64, (usize, Vec<u64>)>,
    position: u64,
    time: usize,
) -> usize {
    if time == 0 {
        return 0
    }

    // check cache
    let key = (time, position.clone(), opened.clone());
    match cache.get(&key) {
        Some(pressure) => return pressure.to_owned(),
        None => {},
    }

    let res = {
        let mut options = Vec::<usize>::new();
        let (rate, leads_to) = valves.get(&position).unwrap();

        if *rate > 0 && !contains(opened, position) {
            options.push(
                (time - 1) * rate + recurse_p1(cache, set(opened, position), valves, position.clone(), time - 1)
            )
        }

        options.append(
            &mut leads_to.iter()
                .map(|location| {
                    recurse_p1(
                        cache,
                        opened.clone(),
                        valves,
                        location.clone(),
                        time - 1,
                    )
                })
                .collect::<Vec<usize>>()
        );

        options.iter().max().cloned().unwrap_or(0usize)
    };

    cache.insert(key, res);
    res
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    let (keys_map, valves) = parse_lines(lines);

    let mut cache = HashMap::<(usize, u64, u64), usize>::new();
    recurse_p1(&mut cache, 0, &valves, keys_map.get("AA").unwrap().to_owned(), 30) as u32
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Open(usize),
    To(u64),
}

impl Move {
    fn pressure(&self) -> usize {
        match self {
            Self::Open(pressure) => *pressure,
            _ => 0,
        }
    }

    fn open(&self, opened: &mut Opened, pos: u64) {
        match self {
            Self::Open(_) => opened.open(pos),
            _ => {},
        }
    }
}

#[derive(Debug, Clone)]
struct Opened {
    val: u64,
    size: usize,
}

impl Opened {
    fn new(size: usize) -> Opened {
        Opened{
            val: 0,
            size,
        }
    }

    fn still_closed(&self) -> Vec<u64> {
        let mut res = Vec::with_capacity(self.size);
        for i in 0..self.size {
            if self.val & (1 << i) == 0 {
                res.push((1 << i) as u64);
            }
        }
        res
    }

    fn is_closed(&self, key: u64) -> bool {
        self.val & key == 0
    }

    fn open(&mut self, key: u64) {
        self.val |= key
    }

    fn key(&self) -> u64 {
        self.val
    }

    fn show(&self) -> String {
        format!("{:b}", self.val)
    }
}

fn recurse_p2(
    cache: &mut HashMap<(usize, u64, u64), usize>,
    opened: Opened,
    valves: &HashMap::<u64, (usize, Vec<u64>)>,
    pos_you: u64,
    from_you: u64,
    pos_elephant: u64,
    from_elephant: u64,
    time: usize,
    pressure_released: usize,
) -> usize {
    if time == 0 {
        return pressure_released
    }

    // check cache
    let key = (time, pos_you | pos_elephant, opened.key());
    match cache.get(&key) {
        Some(pressure) => return pressure.to_owned(),
        None => {},
    }

    let res = {
        let mut moves_you: Vec<Move> = Vec::new();
        let mut already_opened = false;
        let (rate_you, leads_to_you) = valves.get(&pos_you).unwrap();
        if *rate_you > 0 && opened.is_closed(pos_you) {
            moves_you.push(Move::Open((time - 1) * rate_you));
            already_opened = true;
        }
        for new_pos in leads_to_you {
            if *new_pos != from_you {
                moves_you.push(Move::To(*new_pos))
            }
        }

        let mut moves_elephant: Vec<Move> = Vec::new();
        let (rate_elephant, leads_to_elephant) = valves.get(&pos_elephant).unwrap();
        if !((pos_you == pos_elephant) && already_opened) && *rate_elephant > 0 && opened.is_closed(pos_elephant) {
            moves_elephant.push(Move::Open((time - 1) * rate_elephant));
        }
        for new_pos in leads_to_elephant {
            if *new_pos != from_elephant {
                moves_elephant.push(Move::To(*new_pos))
            }
        }

        let mut options = Vec::<usize>::new();

        for move_you in &moves_you {
            for move_elephant in &moves_elephant {
                let pos_you_new = match move_you {
                    Move::Open(_) => pos_you,
                    Move::To(new_pos) => *new_pos,
                };
                let pos_elephant_new = match move_elephant {
                    Move::Open(_) => pos_elephant,
                    Move::To(new_pos) => *new_pos,
                };

                let pressure = move_you.pressure() + move_elephant.pressure();
                let mut new_opened = opened.clone();
                move_you.open(&mut new_opened, pos_you);
                move_elephant.open(&mut new_opened, pos_elephant);
                // let new_opened = move_you.open(move_elephant.open(opened, pos_elephant), pos_you);

                options.push(recurse_p2(
                    cache,
                    new_opened,
                    valves,
                    pos_you_new,
                    pos_you,
                    pos_elephant_new,
                    pos_elephant,
                    time - 1,
                    pressure_released + pressure,
                ));
            }
        }

        // if options.len() >= 5 {
        //     println!("T: {}, n={}, ?={}", time, options.len(), pos_you == pos_elephant);
        // }
        options.iter().max().cloned().unwrap_or(0usize)
    };

    cache.insert(key, res);
    res
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    let (keys_map, valves) = parse_lines(lines);

    let mut cache = HashMap::<(usize, u64, u64), usize>::new();
    let opened = Opened::new(keys_map.len());
    let pos_init = *keys_map.get("AA").unwrap();
    recurse_p2(&mut cache, opened, &valves, pos_init, pos_init, pos_init, pos_init, 26, 0) as u32
}

#[cfg(test)]
mod tests {
    use crate::day16::day16::{Opened, solve_p1, solve_p2};

    #[test]
    fn test_opened() {
        let mut opened = Opened::new(32);

        for i in 0..32 {
            assert_eq!(opened.is_closed(i << 1), true);
        }
        for i in 0..16 {
            let key = 1 << i;
            opened.open(key);
            assert_eq!(opened.is_closed(key), false);
            assert_eq!(opened.is_closed(key << 16), true);
        }
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_owned(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_owned(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_owned(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_owned(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_owned(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_owned(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_owned(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_owned(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_owned(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 1651)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_owned(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_owned(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_owned(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_owned(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_owned(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_owned(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_owned(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_owned(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_owned(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 1707)
    }
}

