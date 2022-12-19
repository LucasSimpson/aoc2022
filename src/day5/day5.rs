use regex::Regex;

#[derive(Debug)]
struct Stacks(Vec<String>);

impl Stacks {
    fn from_lines(lines: Vec<String>) -> Stacks {
        Stacks(transpose(lines)
            .into_iter()
            .skip(1)
            .step_by(4)
            .map(|mut line| {
                // drop the last char by popping
                line.pop();
                line.trim().to_string().chars().rev().collect() // need to reverse here
            }).collect())
    }

    fn do_move(&mut self, from: usize, to: usize) {
        let datum = &mut self.0[from].pop().unwrap();
        let _ = &mut self.0[to].push(*datum);
    }

    fn do_move_rep(&mut self, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            self.do_move(from, to)
        }
    }

    fn do_move_group(&mut self, count: usize, from: usize, to: usize) {
        let mut iter = self.0[from].chars().rev();
        let datum: String = iter.by_ref().take(count).collect::<Vec<char>>().iter().rev().collect();
        self.0[from] = iter.rev().collect();
        self.0[to].push_str(&datum);
    }

    fn tops(&self) -> String {
        self.0.iter().map(|l| l.chars().last().unwrap() ).collect()
    }
}

fn moves_from_lines(lines: Vec<String>) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    lines.into_iter().map(|l| {
        let captures = re.captures(&l).unwrap();

        (
            *&captures[1].parse::<usize>().unwrap(),
            &captures[2].parse::<usize>().unwrap() - 1,
            &captures[3].parse::<usize>().unwrap() - 1,
        )
    }).collect()
}

pub fn solve_p1(lines: Vec<String>) -> String {
    let mut iterator = lines.into_iter();
    let stacks_lines: Vec<String> = iterator.by_ref().take_while(|l| l != "" ).collect();
    let moves_lines: Vec<String> = iterator.clone().collect();

    let mut stacks = Stacks::from_lines(stacks_lines);
    let moves = moves_from_lines(moves_lines);

    for (count, from, to) in moves {
        stacks.do_move_rep(count, from, to);
    }

    stacks.tops()
}

pub fn solve_p2(lines: Vec<String>) -> String {
    let mut iterator = lines.into_iter();
    let stacks_lines: Vec<String> = iterator.by_ref().take_while(|l| l != "" ).collect();
    let moves_lines: Vec<String> = iterator.clone().collect();

    let mut stacks = Stacks::from_lines(stacks_lines);
    let moves = moves_from_lines(moves_lines);

    for (count, from, to) in moves {
        stacks.do_move_group(count, from, to);
    }

    stacks.tops()
}

fn transpose(input: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    // Find the maximum string length.
    let max_len = input.iter().map(|s| s.len()).max().unwrap();

    // Iterate over the original vector, adding each character at a given index
    // to the new vector, until you reach the maximum length.
    for i in 0..max_len {
        let mut transposed = String::new();
        for j in 0..input.len() {
            transposed.push(input[j].chars().nth(i).unwrap_or(' '));
        }
        result.push(transposed);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};
    use crate::day5::day5::transpose;


    struct TC {
        input: Vec<String>,
        expected: Vec<String>,
    }

    #[test]
    fn test_transpose() {
        let test_cases: Vec<TC> = vec![
            TC{
                input: vec![
                    "a b c".to_string(),
                    "a b c".to_string(),
                ],
                expected: vec![
                    "aa".to_string(),
                    "  ".to_string(),
                    "bb".to_string(),
                    "  ".to_string(),
                    "cc".to_string(),
                ]
            },
            TC{
                input: vec![
                    " ".to_string(),
                ],
                expected: vec![
                    " ".to_string(),
                ]
            },
            TC{
                input: vec![
                    "l".to_string(),
                    "u".to_string(),
                    "c".to_string(),
                    "a".to_string(),
                    "s".to_string(),
                ],
                expected: vec![
                    "lucas".to_string(),
                ],
            },
        ];

        for tc in test_cases {
            assert_eq!(transpose(tc.input.clone()), tc.expected);
            assert_eq!(transpose(transpose(tc.input.clone())), tc.input);
        }
    }

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        assert_eq!(solve_p1(lines), "CMZ")
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        assert_eq!(solve_p2(lines), "MCD")
    }
}

