use std::collections::HashSet;
use std::fmt::{Display, Formatter};

fn parse_lines(lines: Vec<String>) -> Vec<isize> {
    lines.into_iter()
        .map(|l| l.parse::<isize>().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Ring {
    nums: Vec<(usize, isize)>,
    size: usize,
}

impl Ring {
    fn new(nums: Vec<isize>) -> Ring {
        let size = nums.len();
        Ring{
            nums: nums.into_iter().enumerate().collect(),
            size,
        }
    }

    fn as_vec(&self) -> Vec<isize> {
        self.nums.iter().map(|(_, x)| *x).collect()
    }

    fn find(&self, value: isize) -> usize {
        // self.nums.iter().find(|(_, val)| *val == value).unwrap().0;
        self.nums.iter().map(|(_, x)| *x).enumerate().find(|(_, val)| *val == value).unwrap().0
    }

    fn get(&self, ind: usize) -> isize {
        self.nums[ind % self.size].1
    }

    fn current_index_of(&self, original_index: usize) -> usize {
        self.nums.iter()
            .map(|(i, _)| i)
            .enumerate()
            .find(|(i, initial)| **initial == original_index)
            .unwrap().0
    }

    fn mix(&mut self, original_index: usize, moves: isize) {
        let mut pos = self.current_index_of(original_index) as isize;

        if moves > 0 {
            let rel_moves = moves % ((self.size - 1) as isize);
            for _ in 0..rel_moves {
                self.swap(pos, pos + 1);
                pos += 1
            }
        } else if moves < 0 {
            let rel_moves = (moves % ((self.size - 1) as isize)).abs();
            for _ in 0..rel_moves {
                self.swap(pos, pos - 1);
                pos -= 1
            }
        }
    }

    fn swap(&mut self, from: isize, to: isize) {
        let size = self.size as isize;
        let rel_from: usize = if from < 0 {
            (from % size) + size
        } else {
            from % size
        } as usize;
        let rel_to: usize = if to < 0 {
            (to % size) + size
        } else {
            to % size
        } as usize;
        self.nums.swap(rel_from, rel_to)
    }
}

impl Display for Ring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nums.iter().map(|(ind, val)| val.to_string()).collect::<Vec<String>>().join(", "))
    }
}

pub fn solve_p1(lines: Vec<String>) -> isize {
    let nums = parse_lines(lines);

    let mut ring = Ring::new(nums.clone());
    for (ind, rot) in nums.into_iter().enumerate() {
        ring.mix(ind, rot);
    }

    let ind = ring.find(0);
    ring.get(ind + 1000) + ring.get(ind + 2000) + ring.get(ind + 3000)
}

pub fn solve_p2(lines: Vec<String>) -> isize {
    let nums = parse_lines(lines).into_iter().map(|x| x * 811589153).collect::<Vec<isize>>();

    let mut ring = Ring::new(nums.clone());
    for _ in 0..10 {
        for (ind, rot) in nums.clone().into_iter().enumerate() {
            ring.mix(ind, rot);
        }
    }

    let ind = ring.find(0);
    ring.get(ind + 1000) + ring.get(ind + 2000) + ring.get(ind + 3000)
}

#[cfg(test)]
mod tests {
    use crate::day20::day20::{Ring, solve_p1, solve_p2};

    fn mix(ring: &Ring, ind: usize, moves: isize) -> Vec<isize> {
        let mut r = ring.clone();
        r.mix(ind, moves);
        r.as_vec()
    }

    #[test]
    fn test_ring_ind() {
        let mut ring = Ring::new(Vec::from([1, 2, 3, 1, 2, 3]));

        assert_eq!(ring.current_index_of(0), 0);
        assert_eq!(ring.current_index_of(1), 1);
        assert_eq!(ring.current_index_of(2), 2);
        assert_eq!(ring.current_index_of(3), 3);
        assert_eq!(ring.current_index_of(4), 4);
        assert_eq!(ring.current_index_of(5), 5);
        ring.mix(0, 3); // [2, 3, 1, 1, 2, 3]
        assert_eq!(ring.as_vec(), Vec::from([2, 3, 1, 1, 2, 3]));
        assert_eq!(ring.current_index_of(0), 3);
        assert_eq!(ring.current_index_of(1), 0);
        assert_eq!(ring.current_index_of(2), 1);
        assert_eq!(ring.current_index_of(3), 2);
        assert_eq!(ring.current_index_of(4), 4);
        assert_eq!(ring.current_index_of(5), 5);
        ring.mix(4, 3); // [3, 2, 1, 1, 3, 2]
        assert_eq!(ring.as_vec(), Vec::from([3, 2, 1, 1, 3, 2]));
        assert_eq!(ring.current_index_of(0), 3);
        assert_eq!(ring.current_index_of(1), 5);
        assert_eq!(ring.current_index_of(2), 0);
        assert_eq!(ring.current_index_of(3), 2);
        assert_eq!(ring.current_index_of(4), 1);
        assert_eq!(ring.current_index_of(5), 4);
    }

    #[test]
    fn test_ring() {
        let ring = &Ring::new(Vec::from([1, 2, 3, 4, 5]));

        assert_eq!(mix(ring, 0, -8), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 0, -7), Vec::from([5, 2, 1, 3, 4]));
        assert_eq!(mix(ring, 0, -6), Vec::from([5, 2, 3, 1, 4]));
        assert_eq!(mix(ring, 0, -5), Vec::from([5, 2, 3, 4, 1]));

        assert_eq!(mix(ring, 0, -4), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 0, -3), Vec::from([5, 2, 1, 3, 4]));
        assert_eq!(mix(ring, 0, -2), Vec::from([5, 2, 3, 1, 4]));
        assert_eq!(mix(ring, 0, -1), Vec::from([5, 2, 3, 4, 1]));

        assert_eq!(mix(ring, 0, 0), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 0, 1), Vec::from([2, 1, 3, 4, 5]));
        assert_eq!(mix(ring, 0, 2), Vec::from([2, 3, 1, 4, 5]));
        assert_eq!(mix(ring, 0, 3), Vec::from([2, 3, 4, 1, 5]));

        assert_eq!(mix(ring, 0, 4), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 0, 5), Vec::from([2, 1, 3, 4, 5]));
        assert_eq!(mix(ring, 0, 6), Vec::from([2, 3, 1, 4, 5]));
        assert_eq!(mix(ring, 0, 7), Vec::from([2, 3, 4, 1, 5]));


        assert_eq!(mix(ring, 3, -8), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 3, -7), Vec::from([4, 1, 2, 3, 5]));
        assert_eq!(mix(ring, 3, -6), Vec::from([1, 4, 2, 3, 5]));
        assert_eq!(mix(ring, 3, -5), Vec::from([1, 2, 4, 3, 5]));

        assert_eq!(mix(ring, 3, -4), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 3, -3), Vec::from([4, 1, 2, 3, 5]));
        assert_eq!(mix(ring, 3, -2), Vec::from([1, 4, 2, 3, 5]));
        assert_eq!(mix(ring, 3, -1), Vec::from([1, 2, 4, 3, 5]));

        assert_eq!(mix(ring, 3, 0), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 3, 1), Vec::from([1, 2, 3, 5, 4]));
        assert_eq!(mix(ring, 3, 2), Vec::from([4, 2, 3, 5, 1]));
        assert_eq!(mix(ring, 3, 3), Vec::from([2, 4, 3, 5, 1]));

        assert_eq!(mix(ring, 3, 4), Vec::from([1, 2, 3, 4, 5]));
        assert_eq!(mix(ring, 3, 5), Vec::from([1, 2, 3, 5, 4]));
        assert_eq!(mix(ring, 3, 6), Vec::from([4, 2, 3, 5, 1]));
        assert_eq!(mix(ring, 3, 7), Vec::from([2, 4, 3, 5, 1]));
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "1".to_owned(),
            "2".to_owned(),
            "-3".to_owned(),
            "3".to_owned(),
            "-2".to_owned(),
            "0".to_owned(),
            "4".to_owned(),
        ];
        assert_eq!(solve_p1(lines), 3)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "1".to_owned(),
            "2".to_owned(),
            "-3".to_owned(),
            "3".to_owned(),
            "-2".to_owned(),
            "0".to_owned(),
            "4".to_owned(),
        ];
        assert_eq!(solve_p2(lines), 1623178306)
    }
}

