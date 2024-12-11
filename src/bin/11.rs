use std::collections::HashMap;

use advent_of_code::{count_digits, ints};

advent_of_code::solution!(11);

fn dfs(s: usize, d: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if d == 0 {
        return 1;
    }
    if let Some(v) = memo.get(&(s, d)) {
        return *v;
    }
    let digits = count_digits(s);
    let ans = if s == 0 {
        dfs(1, d - 1, memo)
    } else if digits % 2 == 0 {
        let m: usize = 10_usize.pow(digits / 2);
        dfs(s / m, d - 1, memo) + dfs(s % m, d - 1, memo)
    } else {
        dfs(s * 2024, d - 1, memo)
    };
    memo.insert((s, d), ans);
    ans
}

pub fn part_one(input: &str) -> Option<u32> {
    let i = ints(input);
    let mut memo = HashMap::new();
    Some(
        i.into_iter()
            .map(|x| dfs(x as usize, 25, &mut memo))
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let i = ints(input);
    let mut memo = HashMap::new();
    Some(
        i.into_iter()
            .map(|x| dfs(x as usize, 75, &mut memo))
            .sum::<usize>() as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // no included test in the problem
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
