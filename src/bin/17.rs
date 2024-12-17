use advent_of_code::{ints, parse_u, uints, vec_lines};
use itertools::Itertools;

advent_of_code::solution!(17);

// my hardcoded input
fn calc(a: usize) -> usize {
    let i = a & 7;
    ((a >> (i ^ 3)) ^ a ^ 6) & 7
}

pub fn part_one(input: &str) -> Option<String> {
    let mut a = uints(input.lines().next().unwrap())[0];
    let mut ans = Vec::new();
    while a > 0 {
        ans.push(calc(a).to_string());
        a >>= 3;
    }
    Some(ans.join(","))
}

fn dfs(a: usize, p: &[usize], i: usize) -> Option<usize> {
    if i >= p.len() {
        return Some(a);
    }
    for x in 0..=7 {
        let na = (a << 3) + x;
        if calc(na) == p[i] {
            let res = dfs(na, p, i+1);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut p = vec_lines(input)[4][9..].split(',').map(parse_u).collect_vec();
    p.reverse();
    dfs(0, &p, 0)
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
