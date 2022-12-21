use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
            .map_err(|_| ())?;
        let captures = re.captures(s).ok_or(())?;

        Ok(Blueprint{
            id: captures[1].parse::<usize>().map_err(|_| ())?,
            ore: captures[2].parse::<usize>().map_err(|_| ())?,
            clay: captures[3].parse::<usize>().map_err(|_| ())?,
            obsidian: (
                captures[4].parse::<usize>().map_err(|_| ())?,
                captures[5].parse::<usize>().map_err(|_| ())?
            ),
            geode: (
                captures[6].parse::<usize>().map_err(|_| ())?,
                captures[7].parse::<usize>().map_err(|_| ())?
            ),
        })
    }
}

fn potential(time: usize) -> usize {
    (time - 1) * time / 2
}

impl Blueprint {
    // get a vector of all possible new states you could transition too
    fn next_states(&self, state: State) -> Vec<State> {
        // get all possible results (do nothing is a default option)
        let res: Vec<State> = state.buy(self, false);

        // apply the collection of original state to the new ones
        res.into_iter()
            .map(|mut s| {
                state.collect_to(&mut s);
                s
            })
            .collect()
    }


    fn solve_most_geodes(&self, cache: &mut HashMap<(State, usize), usize>, state: State, time: usize, best_found: usize) -> usize {
        // if we can't possible beat the best, quit
        match time {
            0 => return state.geodes,  // base case
            1 => return state.geodes + state.geode_rob,  // base again cause I'm cool with it
            2 => {
                let gr_ore = state.ore / self.geode.0;
                let gr_obs = state.obsidian / self.geode.1;
                let gr = cmp::min(gr_ore, gr_obs);
                return state.geodes + (2 * state.geode_rob) + gr;
            },
            _ => {
                if potential(time) + state.geode_rob * time + state.geodes <= best_found {
                    return 0;
                }
            }
        }

        let key = (state.clone(), time);
        if let Some(res) = cache.get(&key) {
            return *res
        }

        let res = {
            // try all combos
            let mut best: usize = best_found;
            let ps = self.next_states(state);
            for next in ps {
                let sub_res = self.solve_most_geodes(cache, next, time - 1, best);
                if sub_res > best {
                    best = sub_res;
                }
            };
            best
        };

        cache.insert(key, res);
        res
    }

    fn most_geodes(&self, total_time: usize) -> u32 {
        let mut cache: HashMap<(State, usize), usize> = HashMap::new();
        let geodes = self.solve_most_geodes(&mut cache, State::new(), total_time, 0);
        geodes as u32
    }

    fn quality(&self, total_time: usize) -> u32 {
        self.id as u32 * self.most_geodes(total_time)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,

    ore_rob: usize,
    clay_rob: usize,
    obsidian_rob: usize,
    geode_rob: usize,
}

impl State {
    fn new() -> State {
        State{
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_rob: 1,
            clay_rob: 0,
            obsidian_rob: 0,
            geode_rob: 0,
        }
    }

    fn buy(&self, bp: &Blueprint, _are_creating: bool) -> Vec<State> {
        let mut res: Vec<State> = Vec::new();
        let mut cb_ore: bool = false;
        let mut cb_clay: bool = false;
        let mut cb_obsidian: bool = false;
        let mut cb_geode: bool = false;

        if self.ore >= bp.ore {
            cb_ore = true;
            let mut opt = self.clone();
            opt.ore -= bp.ore;
            opt.ore_rob += 1;
            res.push(opt);
        }

        if self.ore >= bp.clay {
            cb_clay = true;
            let mut opt = self.clone();
            opt.ore -= bp.clay;
            opt.clay_rob += 1;
            res.push(opt);
        }

        if self.ore >= bp.obsidian.0 && self.clay >= bp.obsidian.1 {
            cb_obsidian = true;
            let mut opt = self.clone();
            opt.ore -= bp.obsidian.0;
            opt.clay -= bp.obsidian.1;
            opt.obsidian_rob += 1;
            res.push(opt);
        }

        if self.ore >= bp.geode.0 && self.obsidian >= bp.geode.1 {
            cb_geode = true;
            let mut opt = self.clone();
            opt.ore -= bp.geode.0;
            opt.obsidian -= bp.geode.1;
            opt.geode_rob += 1;
            res.push(opt);
        }

        if !(cb_ore && cb_clay && cb_obsidian && cb_geode) {
            res.push(self.clone());
        }

        res
    }

    fn collect_to(&self, other: &mut State) {
        other.ore += self.ore_rob;
        other.clay += self.clay_rob;
        other.obsidian += self.obsidian_rob;
        other.geodes += self.geode_rob;
    }

    fn show(&self) -> String {
        format!(
            "[ore: {:>2}, r: {:>2}], [clay: {:>2}, r: {:>2}], [obsidian: {:>2}, r: {:>2}], [geodes: {:>2}, r: {:>2}]",
            self.ore, self.ore_rob, self.clay, self.clay_rob, self.obsidian, self.obsidian_rob, self.geodes, self.geode_rob)
    }
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    lines.into_iter()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .map(|b| b.quality(24))
        .sum()
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    lines.into_iter()
        .take(3)
        .map(|l| l.parse::<Blueprint>().unwrap())
        .map(|b| b.most_geodes(32))
        .product()
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day19::day19::{Blueprint, solve_p1, solve_p2, State};

    #[test]
    fn test_solve_blueprint_1_trivial() {
        let line: String = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_owned();
        let bp = line.parse::<Blueprint>().unwrap();
        let mut cache: HashMap<(State, usize), usize> = HashMap::new();

        let mut state = State::new();
        state.geodes = 4;
        let geodes = bp.solve_most_geodes(&mut cache, state, 0, 0);
        assert_eq!(geodes, 4);

        let mut state = State::new();
        state.geodes = 4;
        state.ore = 2;
        state.obsidian = 14;
        state.geode_rob = 3;
        let geodes = bp.solve_most_geodes(&mut cache, state, 1, 0);
        assert_eq!(geodes, 7);

        let mut state = State::new();
        state.geodes = 4;
        state.ore = 2;
        state.obsidian = 14;
        state.geode_rob = 3;
        let geodes = bp.solve_most_geodes(&mut cache, state, 2, 0);
        assert_eq!(geodes, 11);

        let mut state = State::new();
        state.geodes = 4;
        state.ore = 4;
        state.obsidian = 14;
        state.geode_rob = 3;
        let geodes = bp.solve_most_geodes(&mut cache, state, 2, 0);
        assert_eq!(geodes, 12);
    }

    #[test]
    fn test_solve_blueprint_1() {
        let line: String = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_owned();
        assert_eq!(line.parse::<Blueprint>().unwrap().quality(24), 9)
    }

    #[test]
    fn test_solve_blueprint_2() {
        let line: String = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_owned();
        assert_eq!(line.parse::<Blueprint>().unwrap().quality(24), 24)
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_owned(),
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 33)
    }

    #[test]
    fn test_solve_blueprint_1_long() {
        let line: String = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_owned();
        assert_eq!(line.parse::<Blueprint>().unwrap().most_geodes(32), 56)
    }

    #[test]
    fn test_solve_blueprint_2_long() {
        let line: String = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_owned();
        assert_eq!(line.parse::<Blueprint>().unwrap().most_geodes(32), 62)
    }
}


