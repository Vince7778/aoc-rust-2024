use advent_of_code::{ints, parse};
use itertools::Itertools;

advent_of_code::solution!(7);

// 30x faster than using strings
fn iconcat(a: isize, b: isize) -> isize {
    let mut mul = 1;
    while b >= mul {
        mul *= 10;
    }
    a * mul + b
}

fn dfs(vals: &[isize], target: isize, i: usize, cur: isize, part2: bool) -> bool {
    if i == vals.len() {
        return cur == target;
    }
    dfs(vals, target, i + 1, cur + vals[i], part2)
        || dfs(vals, target, i + 1, cur * vals[i], part2)
        || (part2 && dfs(vals, target, i + 1, iconcat(cur, vals[i]), part2))
}

pub fn part_one(input: &str) -> Option<usize> {
    let ans: isize = input
        .lines()
        .map(|l| {
            let (x, rest) = l.split(':').collect_tuple().unwrap();
            let x = parse(x);
            let r = ints(rest);
            if dfs(&r, x, 0, 0, false) {
                x
            } else {
                0
            }
        })
        .sum();
    Some(ans as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ans: isize = input
        .lines()
        .map(|l| {
            let (x, rest) = l.split(':').collect_tuple().unwrap();
            let x = parse(x);
            let r = ints(rest);
            if dfs(&r, x, 0, 0, true) {
                x
            } else {
                0
            }
        })
        .sum();
    Some(ans as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
