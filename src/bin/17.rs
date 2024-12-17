use advent_of_code::{uints, vec_lines};
use itertools::Itertools;

advent_of_code::solution!(17);

// my hardcoded input simplified
fn calc(a: usize) -> u8 {
    let i = a & 7;
    (((a >> (i ^ 3)) ^ a ^ 6) & 7) as u8
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

fn dfs(a: usize, p: &[u8], i: usize) -> Option<usize> {
    if i >= p.len() {
        return Some(a);
    }
    (0..=7).find_map(|x| {
        let na = (a << 3) + x;
        (calc(na) == p[i]).then(|| dfs(na, p, i + 1)).flatten()
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    let p = vec_lines(input)[4][9..]
        .split(',')
        .map(|x| x.parse().unwrap())
        .rev()
        .collect_vec();
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
