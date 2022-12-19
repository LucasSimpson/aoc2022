use std::collections::HashSet;

fn chars_till_diff(lines: Vec<String>, n: usize) -> u32 {
    lines
        .first()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .take_while(|w| {
            HashSet::<&char>::from_iter(w.iter()).len() != n
        })
        .count() as u32 + n as u32
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    chars_till_diff(lines, 4)
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    chars_till_diff(lines, 14)
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),

        ];
        let sols: Vec<u32> = vec![
            7, 5, 6, 10, 11,
        ];
        lines.into_iter().zip(sols.into_iter()).for_each(|(t, e)| {
            assert_eq!(solve_p1(vec![t]), e)
        });
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),

        ];
        let sols: Vec<u32> = vec![
            19, 23, 23, 29, 26,
        ];
        lines.into_iter().zip(sols.into_iter()).for_each(|(t, e)| {
            assert_eq!(solve_p2(vec![t]), e)
        });
    }
}

