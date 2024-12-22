use std::collections::{HashMap, HashSet};

use advent_of_code::uints;
use itertools::Itertools;

advent_of_code::solution!(22);

fn calc(mut x: isize) -> isize {
    x = (x ^ (x * 64)) % 16777216;
    x = (x ^ (x / 32)) % 16777216;
    x = (x ^ (x * 2048)) % 16777216;
    x
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ans = 0;
    for x in uints(input) {
        let mut x = x as isize;
        for _ in 0..2000 {
            x = calc(x);
        }
        ans += x;
    }
    Some(ans as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut m: HashMap<Vec<isize>, usize> = HashMap::new();
    for x in uints(input) {
        let mut x = x as isize;
        let mut vals = Vec::new();
        for _ in 0..2000 {
            let y = calc(x);
            vals.push(((y % 10) - (x % 10), y % 10));
            x = y;
        }
        let mut seen = HashSet::new();
        for v in vals.windows(4) {
            let vc = v.iter().cloned().map(|x| x.0).collect_vec();
            if seen.contains(&vc) {
                continue;
            }
            seen.insert(vc.clone());
            *m.entry(vc).or_default() += v[3].1 as usize;
        }
    }
    let ans = m.into_values().max().unwrap();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
