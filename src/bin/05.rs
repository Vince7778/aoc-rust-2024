use advent_of_code::{parse_u, repeat_2d};
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pats) = input.split("\n\n").collect_tuple().unwrap();
    let mut reqs = repeat_2d(false, 100, 100);
    for l in rules.lines() {
        let (a, b) = l.split("|").map(parse_u).collect_tuple().unwrap();
        reqs[a][b] = true;
    }
    let ret: usize = pats
        .lines()
        .map(|l| {
            let v = l.split(",").map(parse_u).collect_vec();
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    if reqs[v[j]][v[i]] {
                        return 0;
                    }
                }
            }
            v[v.len() / 2]
        })
        .sum();
    Some(ret as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pats) = input.split("\n\n").collect_tuple().unwrap();
    let mut reqs = repeat_2d(false, 100, 100);
    for l in rules.lines() {
        let (a, b) = l.split("|").map(parse_u).collect_tuple().unwrap();
        reqs[a][b] = true;
    }
    let ret: usize = pats
        .lines()
        .map(|l| {
            let mut v = l.split(",").map(parse_u).collect_vec();
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    if reqs[v[j]][v[i]] {
                        v.sort_by(|&a, &b| reqs[b][a].cmp(&reqs[a][b]));
                        return v[v.len() / 2];
                    }
                }
            }
            0
        })
        .sum();
    Some(ret as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
