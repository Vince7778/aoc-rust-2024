use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let f = input.lines().map(|x| {
        let s: Vec<i64> = x.split_whitespace().map(|v| v.parse().unwrap()).collect_vec();
        (s[0], s[1])
    }).collect_vec();
    let mut l1 = f.iter().map(|(a, _)| *a).collect_vec();
    let mut l2 = f.iter().map(|(_, b)| *b).collect_vec();
    l1.sort(); l2.sort();
    let res: i64 = l1.into_iter().zip(l2.into_iter()).map(|(a, b)| (a-b).abs()).sum();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let f = input.lines().map(|x| {
        let s: Vec<i64> = x.split_whitespace().map(|v| v.parse().unwrap()).collect_vec();
        (s[0], s[1])
    }).collect_vec();
    let mut l1 = f.iter().map(|(a, _)| *a).collect_vec();
    let mut l2 = f.iter().map(|(_, b)| *b).collect_vec();
    let mut c: HashMap<i64, i64> = HashMap::default();
    for x in l2 {
        c.insert(x, *c.get(&x).unwrap_or(&0)+1);
    }
    let res: i64 = l1.into_iter().map(|x| x*(*c.get(&x).unwrap_or(&0))).sum();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
