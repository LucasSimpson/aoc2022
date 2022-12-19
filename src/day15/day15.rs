use std::collections::HashSet;
use regex::Regex;

fn man_dist(p1: &(i32, i32), p2: &(i32, i32)) -> usize {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as usize
}

fn get_locations(lines: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    lines.into_iter()
        .map(|line| {
            let captures = re.captures(&line).unwrap();

            let sensor = (
                (&captures[1]).parse::<i32>().unwrap(),
                (&captures[2]).parse::<i32>().unwrap(),
            );
            let beacon = (
                (&captures[3]).parse::<i32>().unwrap(),
                (&captures[4]).parse::<i32>().unwrap(),
            );

            (sensor, beacon)
        })
        .collect()
}

pub fn solve_p1_with_row(lines: Vec<String>, row: i32) -> u32 {
    let locations = get_locations(lines);

    let offset: i32 = locations.iter()
        .map(|(_, b)| *b)
        .collect::<HashSet<(i32, i32)>>()
        .into_iter()
        .map(|(_, y)| {
            if y == row {
                1
            } else {
                0
            }
        })
        .sum();

    locations.into_iter()
        .filter_map(|(sensor, beacon)| {
            let dist = man_dist(&sensor, &beacon);
            let y_dist = (row - sensor.1).abs() as usize;
            if y_dist >= dist {
                None
            } else {
                let x_dist = (dist - y_dist) as i32;
                Some((sensor.0 - x_dist, sensor.0 + x_dist))
            }
        })
        .reduce(|(leftest, rightest), (left, right)| {
            (
                if left < leftest { left } else { leftest },
                if right > rightest { right } else { rightest },
            )
        })
        .map(|(left, right)| {
            ((right - left) - offset + 1) as u32
        })
        .unwrap()
}

fn find_empty(sensors: Vec<((i32, i32), usize)>, max_x: i32) -> (i32, i32) {
    for x in 0i32..max_x + 1 {
        if x % 100 == 0 {
            println!("{} / {}", x, max_x);
        }
        for y in 0i32..max_x + 1 {
            let mut in_range = false;
            for (sensor, dist) in &sensors {
                let beacon_dist = man_dist(sensor, &(x, y));
                if beacon_dist <= *dist {
                    in_range = true;
                    break;
                }
            }
            if !in_range {
                return (x, y)
            }
        }
    }
    panic!("no empty locations found");
}

fn solve_p2_with_cap(lines: Vec<String>, max_x: i32) -> u128 {
    let sensors: Vec<((i32, i32), usize)> = get_locations(lines).into_iter()
        .map(|(sensor, beacon)| (sensor, man_dist(&sensor, &beacon)))
        .collect();

    let candidates: Vec<(i32, i32)> = sensors.iter()
        .flat_map(|(sensor, distance)| {
            // get a vec of all location JUST on the border of this sensor
            let mut res = Vec::<(i32, i32)>::new();
            let dist = *distance as i32;

            for i in 0..dist + 1 {
                // d=1
                // .....
                // ..#..
                // .#X#.
                // ..#..
                // .....

                // top right
                res.push((sensor.0 + i, sensor.1 + dist - i + 1));
                res.push((sensor.0 + i + 1, sensor.1 + dist - i + 1));

                // bottom right
                res.push((sensor.0 + dist + 1 - i, sensor.1 - i));
                res.push((sensor.0 + dist + 1 - i, sensor.1 - i - 1));

                // bottom left
                res.push((sensor.0 - i, sensor.1 - dist + i - 1));
                res.push((sensor.0 - i - 1, sensor.1 - dist + i - 1));

                // top left
                res.push((sensor.0 - dist - 1 + i, sensor.1 + i));
                res.push((sensor.0 - dist - 1 + i, sensor.1 + i + 1));
            }

            res
        })
        .filter(|(x, y)| *x >= 0 && *x <= max_x && *y >= 0 && *y <= max_x)
        .collect();

    for loc in &candidates {
        let mut in_range = false;
        for (sensor_loc, sense_dist) in &sensors {
            if man_dist(loc, sensor_loc) <= *sense_dist {
                in_range = true;
                break
            }
        }
        if !in_range {
            return (loc.0 as u128) * 4000000 + loc.1 as u128
        }
    }
    panic!("distress beacon location not found")
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    solve_p1_with_row(lines, 2000000)
}

pub fn solve_p2(lines: Vec<String>) -> u128 {
    solve_p2_with_cap(lines, 4000000)
}

#[cfg(test)]
mod tests {
    use crate::day15::day15::{solve_p1_with_row, solve_p2_with_cap};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_owned(),
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_owned(),
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_owned(),
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10".to_owned(),
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_owned(),
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_owned(),
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_owned(),
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_owned(),
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_owned(),
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_owned(),
        ];
        assert_eq!(solve_p1_with_row(lines, 10), 26)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_owned(),
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_owned(),
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_owned(),
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_owned(),
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10".to_owned(),
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_owned(),
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_owned(),
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_owned(),
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_owned(),
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_owned(),
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_owned(),
        ];
        assert_eq!(solve_p2_with_cap(lines, 20), 56000011)
    }
}

