use std::collections::HashSet;

pub fn solve_p1(lines: Vec<String>) -> u32 {
    lines
        .into_iter()
        .map(to_sets)
        .map(|(left, right)| {
            let intersection_size = left.intersection(&right).count();

            if intersection_size == left.len() || intersection_size == right.len() {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn solve_p2(lines: Vec<String>) -> u32 {    lines
    .into_iter()
    .map(to_sets)
    .map(|(left, right)| {
        if left.intersection(&right).count() > 0 {
            1
        } else {
            0
        }
    })
    .sum()
}

fn split_string(data: String, split: &str) -> (String, String) {
    let split_point = data.find(&split).unwrap();
    let replaced = data.replace(split, "");
    let (left, right)= replaced.split_at(split_point);
    (left.to_string(), right.to_string())
}

fn to_set(data: String) -> HashSet<u32> {
    let (left, right) = split_string(data, "-");

    HashSet::from_iter(left.parse::<u32>().unwrap()..right.parse::<u32>().unwrap() + 1)
}

fn to_sets(data: String) -> (HashSet<u32>, HashSet<u32>) {
    let (left, right) = split_string(data, ",");
    (to_set(left), to_set(right))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{solve_p1, solve_p2};
    use crate::day4::day4::to_sets;

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(solve_p1(lines), 2)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(solve_p2(lines), 4)
    }

    #[test]
    fn test_to_sets() {
        assert_eq!(to_sets("11-12,4-8".to_string()), (HashSet::from([11, 12]), HashSet::from([4, 5, 6, 7, 8])))
    }
}
